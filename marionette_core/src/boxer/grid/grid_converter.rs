use std::error::Error;
use std::fmt::Display;
use crate::boxer::grid::{Cell, Grid};
use crate::exported_types::{DisassemblerError};

pub type GridConverterRule<T, V, E> = dyn Fn(T) -> Result<V, E>;

pub trait GridConverter<T: Clone + PartialEq + Display, V: Clone + PartialEq + Display, E: Error + From<T>> {
    fn add_rule(&mut self, rule: Box<GridConverterRule<T, V, E>>);
    fn add_rules(&mut self, rules: Vec<Box<GridConverterRule<T, V, E>>>);

    fn convert_cell(&self, o: Cell<T>) -> Result<V, E>;
    fn convert(&self, grid: Grid<T>) -> Result<Grid<V>, E>;
}

/// A simple grid converter that converts a grid of one type to another type.
pub struct SimpleGridConverter<T: Clone + PartialEq + Display, V: Clone + PartialEq + Display, E: Error + From<T>> {
    rules: Vec<Box<GridConverterRule<T, V, E>>>,
}

impl Default for SimpleGridConverter<i16, char, DisassemblerError> {
    /// Creates a new simple grid converter that converts a grid of i16 to a grid of char.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    ///
    /// let mut converter = SimpleGridConverter::<i16, char, DisassemblerError>::default();
    /// ```
    fn default() -> Self {
        SimpleGridConverter::new()
    }
}

impl<T: Clone + PartialEq + Display, V: Clone + PartialEq + Display, E: Error + From<T>> SimpleGridConverter<T, V, E> {
    /// Creates a new simple grid converter that converts a grid of one type to another type.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    ///
    /// let mut converter = SimpleGridConverter::<i16, i32, DisassemblerError>::new();
    /// ```
    pub fn new() -> SimpleGridConverter<T, V, E> {
        SimpleGridConverter {
            rules: Vec::new(),
        }
    }
}

impl<T: Clone + PartialEq + Display, V: Clone + PartialEq + Display, E: Error + From<T>> GridConverter<T, V, E> for SimpleGridConverter<T, V, E> {
    /// Adds a rule to the grid converter.
    /// # Arguments
    /// * `rule` - A rule to add to the grid converter.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    ///
    /// let mut converter = SimpleGridConverter::<i16, i32, DisassemblerError>::new();
    /// converter.add_rule(Box::new(|x| Ok(x as i32)));
    /// ```
    fn add_rule(&mut self, rule: Box<dyn Fn(T) -> Result<V, E>>) {
        self.rules.push(rule);
    }

    /// Adds multiple rules to the grid converter.
    /// # Arguments
    /// * `rules` - A vector of rules to add to the grid converter.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    ///
    /// let mut converter = SimpleGridConverter::<i16, i32, DisassemblerError>::new();
    /// converter.add_rules(vec![
    ///     Box::new(|x| {
    ///         if x == 0 {
    ///             Ok(100)
    ///         } else {
    ///             Err(DisassemblerError::not_implemented(0, "Not implemented"))
    ///         }
    ///     }),
    ///     Box::new(|x| {
    ///         if x != 0 {
    ///             Ok(x * 2 as i32)
    ///         } else {
    ///             Err(DisassemblerError::not_implemented(0, "Not implemented"))
    ///         }
    ///     }),
    /// ]);
    /// ```
    fn add_rules(&mut self, rules: Vec<Box<dyn Fn(T) -> Result<V, E>>>) {
        for rule in rules {
            self.rules.push(rule);
        }
    }

    /// Converts a cell's value to another type.
    /// # Arguments
    /// * `o` - A cell to convert.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    ///
    /// let mut converter = SimpleGridConverter::<i16, i32, DisassemblerError>::new();
    /// converter.add_rules(vec![
    ///     Box::new(|x| x as i32)
    /// ]);
    ///
    /// let cell = converter.convert_cell(Cell::new(0, 0, 100)).unwrap();
    /// assert_eq!(cell, 100);
    /// ```
    fn convert_cell(&self, o: Cell<T>) -> Result<V, E> {
        for rule in &self.rules {
            let result = rule(o.value.clone());
            if result.is_ok() {
                return result;
            }
        }
        Err(E::from(o.value))
    }

    /// Converts a grid's values to a grid of another type.
    /// # Arguments
    /// * `grid` - A grid to convert.
    /// # Example
    /// ```
    /// use marionette_core::exported_types::DisassemblerError;
    /// use marionette_core::boxer::grid::grid_converter::SimpleGridConverter;
    /// use marionette_core::boxer::grid::Grid;
    /// 
    /// let mut converter = SimpleGridConverter::<i16, i32, DisassemblerError>::new();
    /// converter.add_rules(vec![
    ///     Box::new(|x| x as i32)
    /// ]);
    /// 
    /// let grid = Grid::new(1, 1, 100);
    /// let new_grid = converter.convert(grid).unwrap();
    /// assert_eq!(new_grid.get_cell(0, 0).unwrap().value, 100);
    /// ```
    fn convert(&self, grid: Grid<T>) -> Result<Grid<V>, E> {
        let old_default = grid.default_value.clone();
        let mut new_grid = Grid::new(
            grid.width, grid.height,
            V::clone(&self.convert_cell(Cell::new(0, 0, old_default))?)
        );
        for y in 0..grid.height {
            for x in 0..grid.width {
                let old_cell = grid.get_cell(x, y).unwrap().direct_copy();
                let new_cell = Cell::new(x, y, self.convert_cell(old_cell)?);
                new_grid.set_cell(x, y, new_cell.value);
            }
        }
        Ok(new_grid)
    }
}