use std::fmt::Display;
use std::ops::Index;
use crate::boxer;
use crate::boxer::Boxer;

pub mod grid_converter;

#[derive(Clone)]
pub struct Cell<T: PartialEq + Display> where T: Clone {
    pub x: i32,
    pub y: i32,
    pub value: T,
}

impl <T: Clone + PartialEq + Display> Cell<T> where T: Clone {
    pub fn new(x: i32, y: i32, value: T) -> Cell<T> {
        Cell { x, y, value, }
    }

    pub fn direct_copy(&self) -> Cell<T> {
        Cell { x: self.x, y: self.y, value: self.value.clone() }
    }

    pub fn is_at(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

#[derive(Clone)]
pub struct Grid<T: Clone + PartialEq + Display> where Cell<T>: Clone {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<Cell<T>>>,

    pub default_value: T
}

impl<T: Clone + PartialEq + Display> Grid<T> where Cell<T>: Clone {
    /// Creates a new grid with the given width and height, and a default value for each cell.
    /// # Arguments
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    /// * `default_value` - The default value for each cell.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let grid = Grid::new(10, 10, 0);
    /// ```
    pub fn new(width: i32, height: i32, default_value: T) -> Grid<T> {
        let mut cells = Vec::new();
        for y in 0..height {
            cells.push(Vec::new());
            for x in 0..width {
                let cell_value = default_value.clone();
                cells[y as usize].push(Cell {
                    x, y,
                    value: cell_value,
                });
            }
        }
        Grid {
            width, height,
            cells, default_value,
        }
    }

    /// Creates a new grid with the given width and height, and takes in a unsorted vector of cells along with a default value for each cell.
    /// # Arguments
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    /// * `cells` - The unsorted vector of cells.
    /// * `default_value` - The default value for each cell.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let grid = Grid::from_unsorted(2, 2, vec![
    ///     0, 1,
    ///     2, 3,
    ///     4, 5,
    ///     6, 7,
    ///     8, 9
    /// ], 0);
    /// ```
    pub fn from_unsorted(width: i32, height: i32, cells: Vec<T>, default_value: T) -> Grid<T> {
        let mut grid = Grid::new(width, height, default_value);
        for (i, cell) in cells.iter().enumerate() {
            let x = (i as i32 % width) as i32;
            let y = (i as i32 / width) as i32;
            grid.set_cell(x, y, cell.clone());
        }
        grid
    }

    /// Retrieves the cell at the given x and y coordinates otherwise returns None.
    /// # Arguments
    /// * `x` - The x coordinate of the cell.
    /// * `y` - The y coordinate of the cell.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let grid = Grid::new(10, 10, 0);
    /// let cell = grid.get_cell(0, 0);
    /// assert!(cell.is_some());
    /// ```
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell<T>> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(&self.cells[y as usize][x as usize])
    }

    /// Sets the value of the cell at the given x and y coordinates.
    /// # Arguments
    /// * `x` - The x coordinate of the cell.
    /// * `y` - The y coordinate of the cell.
    /// * `value` - The value to set the cell to.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    /// grid.set_cell(0, 0, 1);
    /// assert_eq!(grid.get_cell(0, 0).unwrap().value, 1);
    /// ```
    pub fn set_cell(&mut self, x: i32, y: i32, value: T) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        self.cells[y as usize][x as usize].value = value;
    }

    /// Shifts the x-coordinates of all cells by the given amount.
    /// # Arguments
    /// * `amount` - The amount to shift the x-coordinates by.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    /// grid.shift_x_by(1);
    /// // Due to behavior this doesn't change the internal grid x-coordinates
    /// // but it changes the x-coordinates of the cells.
    /// assert_eq!(grid.get_cell(0, 0).unwrap().x, 1);
    /// ```
    pub fn shift_x_by(&mut self, amount: i32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y as usize][x as usize].x += amount;
            }
        }
    }

    /// Shifts the y-coordinates of all cells by the given amount.
    /// # Arguments
    /// * `amount` - The amount to shift the y-coordinates by.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    /// grid.shift_y_by(1);
    /// // Due to behavior this doesn't change the internal grid y-coordinates
    /// // but it changes the y-coordinates of the cells.
    /// assert_eq!(grid.get_cell(0, 0).unwrap().y, 1);
    /// ```
    pub fn shift_y_by(&mut self, amount: i32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y as usize][x as usize].y += amount;
            }
        }
    }

    /// Adds an empty / default column of cells to the left of the grid.
    /// # Arguments
    /// * `columns` - The amount of columns to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// // The behavior of this function incorporates shifting the x-coordinates
    /// // however at the same time it changes the internal grid width.
    /// assert_eq!(grid.get_cell(10, 0).is_some(), false);
    /// grid.add_empty_column_left(1);
    /// assert_eq!(grid.get_cell(10, 0).is_some(), true);
    /// ```
    pub fn add_empty_column_left(&mut self, columns: i32) {
        self.shift_x_by(columns);
        self.width += columns;
        for y in 0..self.height {
            for _ in 0..columns {
                let cell_value = self.default_value.clone();
                self.cells[y as usize].insert(0, Cell {
                    x: 0, y,
                    value: cell_value,
                });
            }
        }
    }

    /// Adds an empty / default column of cells to the right of the grid.
    /// # Arguments
    /// * `columns` - The amount of columns to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// // Since this function is adding columns to the right of the grid
    /// // the behavior doesn't require shifting the x-coordinates.
    /// assert_eq!(grid.get_cell(10, 0).is_some(), false);
    /// grid.add_empty_column_right(1);
    /// assert_eq!(grid.get_cell(10, 0).is_some(), true);
    /// ```
    pub fn add_empty_column_right(&mut self, columns: i32) {
        self.width += columns;
        for y in 0..self.height {
            for _ in 0..columns {
                let cell_value = self.default_value.clone();
                self.cells[y as usize].push(Cell {
                    x: self.width - 1, y,
                    value: cell_value,
                });
            }
        }
    }

    /// Adds an empty / default row of cells to the top of the grid.
    /// # Arguments
    /// * `rows` - The amount of rows to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// // The behavior of this function incorporates shifting the y-coordinates
    /// // however at the same time it changes the internal grid height.
    /// assert_eq!(grid.get_cell(0, 10).is_some(), false);
    /// grid.add_empty_row_top(1);
    /// assert_eq!(grid.get_cell(0, 10).is_some(), true);
    /// ```
    pub fn add_empty_row_top(&mut self, rows: i32) {
        self.shift_y_by(rows);
        self.height += rows;
        for _ in 0..rows {
            let mut row = Vec::new();
            for x in 0..self.width {
                let cell_value = self.default_value.clone();
                row.push(Cell {
                    x, y: 0,
                    value: cell_value,
                });
            }
            self.cells.insert(0, row);
        }
    }

    /// Adds an empty / default row of cells to the bottom of the grid.
    /// # Arguments
    /// * `rows` - The amount of rows to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// // Since this function is adding rows to the bottom of the grid
    /// // the behavior doesn't require shifting the y-coordinates.
    /// assert_eq!(grid.get_cell(0, 10).is_some(), false);
    /// grid.add_empty_row_bottom(1);
    /// assert_eq!(grid.get_cell(0, 10).is_some(), true);
    /// ```
    pub fn add_empty_row_bottom(&mut self, rows: i32) {
        self.height += rows;
        for _ in 0..rows {
            let mut row = Vec::new();
            for x in 0..self.width {
                let cell_value = self.default_value.clone();
                row.push(Cell {
                    x, y: self.height - 1,
                    value: cell_value,
                });
            }
            self.cells.push(row);
        }
    }

    /// Stretches the grid to the left by adding empty columns to the left of the grid.
    /// then copies the values of the old leftmost column before the stretch into the new empty columns
    /// # Arguments
    /// * `amount` - The amount of columns to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// grid.set_cell(0, 0, 1);
    /// grid.stretch_left(1);
    /// assert_eq!(grid.get_cell(0, 0).unwrap().value, 1);
    /// assert_eq!(grid.get_cell(1, 0).unwrap().value, 1);
    /// ```
    pub fn stretch_left(&mut self, amount: i32) {
        let mut old_leftmost_column = Vec::new();
        for y in 0..self.height {
            old_leftmost_column.push(self.cells[y as usize][0].value.clone());
        }

        self.add_empty_column_left(amount);
        for y in 0..self.height {
            for x in 0..amount {
                self.set_cell(x, y, old_leftmost_column[y as usize].clone());
            }
        }
    }

    /// Stretches the grid to the right by adding empty columns to the right of the grid.
    /// then copies the values of the old rightmost column before the stretch into the new empty columns
    /// # Arguments
    /// * `amount` - The amount of columns to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(10, 10, 0);
    ///
    /// grid.set_cell(9, 0, 1);
    /// grid.stretch_right(1);
    /// assert_eq!(grid.get_cell(9, 0).unwrap().value, 1);
    /// assert_eq!(grid.get_cell(10, 0).unwrap().value, 1);
    /// ```
    pub fn stretch_right(&mut self, amount: i32) {
        let mut old_rightmost_column = Vec::new();
        for y in 0..self.height {
            old_rightmost_column.push(self.cells[y as usize][self.width as usize - 1].value.clone());
        }

        self.add_empty_column_right(amount);
        for y in 0..self.height {
            for x in 0..amount {
                self.set_cell(self.width - 1 - x, y, old_rightmost_column[y as usize].clone());
            }
        }
    }

    /// Compares the values of two grids and returns a vector of tuples containing the cells that differ.
    /// The behavior of this function assumes that the two grids have the same dimensions.
    /// # Arguments
    /// * `other` - The grid to compare against.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::from_unsorted(2, 2, vec![1, 0, 0, 1], 0);
    /// let mut other = Grid::from_unsorted(2, 2, vec![1, 0, 0, 0], 0);
    ///
    /// let diff = grid.diff(other);
    /// assert_eq!(diff.len(), 1);
    /// assert_eq!(diff[0].0.x, 1);
    /// assert_eq!(diff[0].0.y, 1);
    /// ```
    pub fn diff(&mut self, other: Grid<T>) -> Vec<(Cell<T>, Cell<T>)> {
        let mut diff = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                let other_cell = other.get_cell(x, y).unwrap();
                if cell.value != other_cell.value {
                    diff.push((cell.direct_copy(), other_cell.direct_copy()));
                }
            }
        }
        diff
    }

    /// Accepts a vector of cells and sets the values of the cells in the grid to the values of the cells in the vector.
    /// # Arguments
    /// * `resolutions` - The vector of cells to accept.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(2, 2, 0);
    /// let mut resolutions = Vec::new();
    /// resolutions.push(grid.get_cell(0, 0).unwrap().clone());
    /// resolutions[0].value = 1;
    ///
    /// grid.accept_resolved_disputes(resolutions);
    /// assert_eq!(grid.get_cell(0, 0).unwrap().value, 1);
    /// ```
    pub fn accept_resolved_disputes(&mut self, resolutions: Vec<Cell<T>>) {
        for cell in resolutions {
            self.set_cell(cell.x, cell.y, cell.value.clone());
        }
    }

    /// Displays the grid in the terminal.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(2, 2, 0);
    /// grid.output();
    /// ```
    pub fn output(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                print!("{}", cell.value);
            }
            println!();
        }
    }

    /// Displays the grid in the terminal in a grid form.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(2, 2, 0);
    /// grid.output_in_grid_form();
    /// ```
    pub fn output_in_grid_form(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                print!("{} ", cell.value);
            }
            println!();
        }
    }

    /// Displays the grid in the terminal in a grid form with the indexes of the cells.
    /// # Example
    /// ```
    /// use marionette_core::boxer::grid::Grid;
    ///
    /// let mut grid = Grid::new(2, 2, 0);
    /// grid.output_indexes();
    /// ```
    pub fn output_indexes(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                print!("({}, {}) ", cell.x, cell.y);
            }
            println!();
        }
    }
}

