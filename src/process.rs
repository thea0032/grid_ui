use crate::{grid::{Grid, Alignment, DividerStrategy}, out::{Action, Handler, SafeHandler}, trim::{TrimmedText, FormatError, TrimStrategy}};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InternalFormatError {
    NoSpace(TrimmedText),
}
/// A structure that can display text inside a grid.  
/// Cloning chunk processes is bad practice! Use it only if you have to.  
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DrawProcess {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    divider: usize,
    minus: Vec<TrimmedText>,
    plus: Vec<TrimmedText>,
    example_str: String,
}
impl DrawProcess {
    #[doc(hidden)]
    /// Creates a new chunk process.
    pub(crate) fn new(val: Grid, strategy: DividerStrategy) -> DrawProcess {
        DrawProcess {
            start_x: val.start_x,
            start_y: val.start_y,
            end_x: val.end_x,
            end_y: val.end_y,
            divider: match strategy {
                DividerStrategy::Beginning => 0,
                DividerStrategy::End => val.end_y - val.start_y,
                DividerStrategy::Halfway => (val.end_y - val.start_y) / 2,
                DividerStrategy::Pos(v) => v,
            },
            minus: Vec::new(),
            plus: Vec::new(),
            example_str: " ".chars().cycle().take(val.end_x - val.start_x).collect(),
        }
    }
    /// Gets the chunk's width - the number of characters that can be displayed on a line.
    /// ``` rust
    /// # use grid_ui::grid;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    /// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    /// assert_eq!(process.width(), 70);
    /// # Ok(())
    /// # }
    /// ```
    pub fn width(&self) -> usize {
        self.end_x - self.start_x
    }
    /// Gets the chunk's height - the number of lines that can fit in it.
    /// ``` rust
    /// # use grid_ui::grid;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    /// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    /// assert_eq!(process.height(), 70);
    /// # Ok(())
    /// # }
    /// ```
    pub fn height(&self) -> usize {
        self.end_y - self.start_y
    }
    /// Gets the x position where the process begins.
    /// ``` rust
    /// # use grid_ui::grid;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    /// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    /// assert_eq!(process.start_x(), 30);
    /// # Ok(())
    /// # }
    /// ```
    pub fn start_x(&self) -> usize {
        self.start_x
    }

    /**
    Gets the y position where the process begins.
    ``` rust
    # use grid_ui::grid;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    assert_eq!(process.start_y(), 30);
    # Ok(())
    # }
    ```
    */
    pub fn start_y(&self) -> usize {
        self.start_y
    }

    /**
    Gets the x position where the process ends.
    ``` rust
    # use grid_ui::grid;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    assert_eq!(process.end_x(), 100);
    # Ok(())
    # }
    ```
    */
    pub fn end_x(&self) -> usize {
        self.end_x
    }

    /**
    Gets the y position where the process ends.
    ``` rust
    # use grid_ui::grid;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(30, 30, 100, 100).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    assert_eq!(process.end_y(), 100);
    # Ok(())
    # }
    ```
    */
    pub fn end_y(&self) -> usize {
        self.end_y
    }
    #[doc(hidden)]
    /// Trims a string using a trim strategy.
    fn trim<T: TrimStrategy>(&self, text: T::Input, b: &mut T, a: Alignment) -> Vec<TrimmedText> {
        b.trim(text, self, a)
    }
    /**
    Adds multi-line content to the selection, using the inputted strategy inside the inputted alignment. Returns everything that can't fit.
    Note that the multi-line content goes top to bottom, even if Alignment::Minus is selected.
    This is the exact opposite behavior of simply sending multiple lines.
    The content only needs to be iterated through.
    # Errors
    Each position represents the corresponding position of the text input. An error will be found if the call to add_to_section() returns an error.
    # Examples
    Usage in positive direction
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section_lines(vec!["Some stuff".to_string(), "More stuff".to_string()].into_iter(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("Some stuff\nMore stuff\n          \n".to_string(), output);
    # Ok(())
    # }
    ```
    Usage in negative direction
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::End);
    process.add_to_section_lines(vec!["Some stuff".to_string(), "More stuff".to_string()].into_iter(), &mut Ignore, grid::Alignment::Minus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("          \nSome stuff\nMore stuff\n".to_string(), output);
    # Ok(())
    # }
    ```
    Errors in positive direction
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 2).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    let result = process.add_to_section_lines(vec!["Some stuff".to_string(), "More stuff".to_string(), "Even more!".to_string()].into_iter(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("Some stuff\nMore stuff\n".to_string(), output);
    assert!(result[2].is_err());
    # Ok(())
    # }
    ```
    Errors in negative direction
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 2).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::End);
    let result = process.add_to_section_lines(vec!["Some stuff".to_string(), "More stuff".to_string(), "Even more!".to_string()].into_iter(), &mut Ignore, grid::Alignment::Minus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("More stuff\nEven more!\n".to_string(), output);
    assert!(result[0].is_err());
    # Ok(())
    # }
    ```
    */
    pub fn add_to_section_lines<T, I>(&mut self, text: I, strategy: &mut T, section: Alignment) -> Vec<Result<(), FormatError<T>>>
    where
        T: TrimStrategy,
        I: DoubleEndedIterator,
        I: Iterator<Item = T::Input>,
    {
        if matches!(section, Alignment::Minus) {
            let text = text.rev();
            let mut res = text.map(|x| self.add_to_section(x, strategy, section)).collect::<Vec<_>>();
            if matches!(section, Alignment::Minus) {
                res.reverse();
            }
            res
        } else {
            let mut res = text.map(|x| self.add_to_section(x, strategy, section)).collect::<Vec<_>>();
            if matches!(section, Alignment::Minus) {
                res.reverse();
            }
            res
        }
    }
    /**
    Adds single-line content to the selection, using the inputted strategy inside the inputted alignment.
    # Errors
    This method will return an error if the text won't fit. The text will be returned (although it might be trimmed from trim methods.)
    # Examples
    Basic printing:
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("Some stuff\n          \n          \n".to_string(), output);
    # Ok(())
    # }
    ```
    How order works
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    process.add_to_section("More stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("Some stuff\nMore stuff\n          \n".to_string(), output);
    # Ok(())
    # }
    ```
    Running out of space:
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 1).next_frame(); // creates a grid with one line
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    assert!(process.add_to_section("No more".to_string(), &mut Ignore, grid::Alignment::Plus).is_err());
    # Ok(())
    # }
    ```
    Divider strategies can determine where you have space in which section.
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 100, 100).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    assert!(process.add_to_section(
        "The divider is at the beginning! There's no room for negatively aligned text!"
        .to_string(),
        &mut Ignore, grid::Alignment::Minus).is_err());
    # Ok(())
    # }
    ```
    */
    pub fn add_to_section<T: TrimStrategy>(&mut self, text: T::Input, strategy: &mut T, section: Alignment) -> Result<(), FormatError<T>> {
        let text = self.trim(text, strategy, section);
        let mut i = text.into_iter();
        let error: InternalFormatError = loop {
            if let Some(val) = i.next() {
                // If there's more trimmed text...
                if let Err(e) = self.add_to_section_trimmed(val, section) {
                    // Adds it to the section. If an error occurs, break out of the loop.
                    break e;
                }
            } else {
                // If we successfully made it through, we're ok.
                return Ok(());
            }
        };
        match error {
            InternalFormatError::NoSpace(back) => {
                // Adds the text that couldn't be formatted back onto the start and collects them all.
                let extras = Some(back).into_iter().chain(i).collect::<Vec<_>>();
                // Adds the error.
                Err(FormatError::NoSpace(strategy.back(extras, &self, section)))
            }
        }
    }
    /**
    Clears the process, allowing it to be re-used. 
    # Example
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 1).next_frame(); // creates a grid with one line
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    assert!(process.add_to_section("No more".to_string(), &mut Ignore, grid::Alignment::Plus).is_err());
    process.clear(grid::DividerStrategy::Beginning);
    assert!(process.add_to_section("Some more".to_string(), &mut Ignore, grid::Alignment::Plus).is_ok());
    # Ok(())
    # }
    ```
    */
    pub fn clear(&mut self, new_strategy: DividerStrategy) {
        *self = DrawProcess::new(Grid {
            start_x: self.start_x,
            start_y: self.start_y,
            end_x: self.end_x,
            end_y: self.end_y
        }, new_strategy);
    } 
    /**
    Gives up free space in the Y direction, producing a grid if there's free space to give up. 
    Will take up to max_taken lines of space. If max_taken is set to None, it will take up to the divider line. 
    Will leave at least min_left lines TOTAL (in either direction). Might leave some blank lines. 
    Returns this space in a grid if there is any. If the process is already full, nothing will be returned. 
    # Example
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 10).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    assert_eq!(process.split_free_space(grid::Alignment::Plus, None, None), Some(grid::Grid{start_x: 0, start_y: 1, end_x: 10, end_y: 10}));
    assert_eq!(process.end_y(), 1);
    # Ok(())
    # }
    ```
    */
    pub fn split_free_space(&mut self, a: Alignment, min_left: Option<usize>, max_taken: Option<usize>) -> Option<Grid> {
        match a {
            Alignment::Minus => {
                let space = self.divider;
                let mut space_occupied = self.minus.len();
                if let Some(val) = min_left {
                    space_occupied = space_occupied.max(val);
                }
                let mut total_space = space.checked_sub(space_occupied).unwrap_or(0);
                if let Some(val) = max_taken {
                    total_space = total_space.min(val);
                }
                if total_space != 0 {
                    self.start_y += total_space;
                    Some(Grid {
                        start_x: self.start_x,
                        start_y: self.start_y - total_space,
                        end_x: self.end_x,
                        end_y: self.end_y - total_space,
                    })
                } else {
                    None
                }
            },
            Alignment::Plus => {
                println!("END: {} START: {} DIVIDE: {}", self.end_y, self.start_y, self.divider);
                let space = self.end_y - self.start_y - self.divider;
                println!("SPACE: {}", space);
                let mut space_occupied = self.plus.len();
                println!("OCCUPIED: {}", space_occupied);
                if let Some(val) = min_left {
                    space_occupied = space_occupied.max(val);
                }
                let mut total_space = space.checked_sub(space_occupied).unwrap_or(0);
                println!("TOTAL: {}", total_space);
                if let Some(val) = max_taken {
                    total_space = total_space.min(val);
                }
                if total_space != 0 {
                    self.end_y -= total_space;
                    Some(Grid {
                        start_x: self.start_x,
                        start_y: self.end_y,
                        end_x: self.end_x,
                        end_y: self.end_y + total_space,
                    })
                } else {
                    None
                }
            },
        }
    }
    /**
    Extends the grid in the either direction, either positive or negative, if the input is compatible
    (ie grids are next to each other and of similar dimensions)
    If the two grids are incompatible, it returns an error and gives the grid back. 
    # Example
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 10).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    let free_space = process.split_free_space(grid::Alignment::Plus, None, None).ok_or(())?;
    assert_eq!(process.end_y(), 1);
    assert!(process.extend(free_space).is_ok());
    assert_eq!(process.end_y(), 10);
    let incompatible_grid = grid::Frame::new(4, 4, 8, 8).next_frame();
    assert!(process.extend(incompatible_grid).is_err());
    # Ok(())
    # }
    ```
    */
    pub fn extend(&mut self, grid: Grid) -> Result<(), Grid> {
        if self.start_x == grid.start_x && self.end_x == grid.end_x {
            if self.end_y == grid.start_y {
                self.end_y = grid.end_y;
                return Ok(())
            }
            if self.start_y == grid.end_y {
                self.start_y = grid.start_y;
                return Ok(())
            }
        }
        Err(grid)
    }
    #[doc(hidden)]
    /// Adds trimmed text to a section.
    fn add_to_section_trimmed(&mut self, text: TrimmedText, section: Alignment) -> Result<(), InternalFormatError> {
        if matches!(section, Alignment::Minus) {
            let space = self.divider - self.minus.len();
            if space == 0 {
                return Err(InternalFormatError::NoSpace(text));
            }
            self.minus.push(text);
        } else {
            let space = self.end_y - self.start_y - self.divider - self.plus.len();
            if space == 0 {
                return Err(InternalFormatError::NoSpace(text));
            }
            self.plus.push(text);
        }
        Ok(())
    }
    #[doc(hidden)]
    /**
    Shoves the data in the positive or negative direction, changing the divider to make more space available on one side.
    Moving text to the bottom or top:
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 4).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Halfway);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    process.add_to_section("More stuff".to_string(), &mut Ignore, grid::Alignment::Minus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("          \nMore stuff\nSome stuff\n          \n".to_string(), output);
    process.shove(grid::Alignment::Minus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("More stuff\nSome stuff\n          \n          \n".to_string(), output);
    assert!(process.add_to_section("No room left".to_string(), &mut Ignore, grid::Alignment::Minus).is_err());
    process.shove(grid::Alignment::Plus);
    process.add_to_section("More room!".to_string(), &mut Ignore, grid::Alignment::Minus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("          \nMore room!\nMore stuff\nSome stuff\n".to_string(), output);
    # Ok(())
    # }
    ```
    */
    pub fn shove(&mut self, direction: Alignment) {
        match direction {
            Alignment::Minus => self.divider = self.divider.min(self.minus.len()),
            Alignment::Plus => self.divider = self.divider.max(self.end_y - self.start_y - self.plus.len()),
        }
    }
    #[doc(hidden)]
    /// Transforms the board into actions.
    fn grab_actions(&mut self) -> Vec<Action> {
        let mut result = Vec::new();
        let start_x = self.start_x;
        let start_y = self.start_y + self.divider - self.minus.len();
        let divider = self.start_y + self.divider;
        // Adds blank lines, making sure that the entirety of grid is clear.
        for i in self.start_y..start_y {
            result.push(Action::MoveTo(start_x, i));
            result.push(Action::Print(&self.example_str));
        }
        // Adds negative lines
        for (i, line) in self.minus.iter().rev().enumerate() {
            result.push(Action::MoveTo(start_x, start_y + i));
            result.push(Action::Print(&line.0));
        }
        // Adds positive lines
        for (i, line) in self.plus.iter().enumerate() {
            result.push(Action::MoveTo(start_x, divider + i));
            result.push(Action::Print(&line.0));
        }
        // Adds blank lines, making sure that the entirety of grid is clear.
        for i in self.start_y + self.divider + self.plus.len()..self.end_y {
            result.push(Action::MoveTo(start_x, i));
            result.push(Action::Print(&self.example_str));
        }
        result
    }
    /**
    Prints out the grid using a handler.
    # Errors
    Returns an error if the handler returns an error.
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print(&mut out::OutToString, &mut output)?;
    assert_eq!("Some stuff\n          \n          \n".to_string(), output);
    # Ok(())
    # }
    ```
    */
    pub fn print<H: Handler>(&mut self, handler: &mut H, out: &mut H::OutputDevice) -> Result<(), H::Error> {
        let actions = self.grab_actions();
        for line in actions {
            handler.handle(out, &line)?;
        }
        Ok(())
    }
    /**
    Prints safely - this method cannot return an error.
    # Panics
    This method panics when the handler panics.
    # Examples
    Safe printing:
    ``` rust
    # use grid_ui::grid;
    # use grid_ui::out;
    # use grid_ui::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
    let mut process = grid.into_process(grid::DividerStrategy::Beginning);
    process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
    let mut output: String = String::new();
    process.print_safe(&mut out::OutToString, &mut output);
    assert_eq!("Some stuff\n          \n          \n".to_string(), output);
    # Ok(())
    # }
    ```
    */
    pub fn print_safe<H: SafeHandler>(&mut self, handler: &mut H, out: &mut H::OutputDevice) {
        let actions = self.grab_actions();
        for line in actions {
            handler.safe_handle(out, &line);
        }
    }
}
