// Purpose: boxer is the internal library for marionette that is responsible
// for generating graphical jump lines from disassembly jump data generated
// by boxer from jumps sent to it by the disassembler
// Path: src\boxer.rs

use crate::boxer::dispute_resolver::{DisputeResolverTrait, SimpleDisputeResolver};
pub use crate::boxer::dispute_resolver::dispute_rule::{DisputeRule};
use crate::boxer::grid::{Cell, Grid};
use crate::boxer::grid::grid_converter::{GridConverter, SimpleGridConverter};
use crate::exported_types::{DisassemblerError, DisassemblerErrorType};

pub mod grid;
#[macro_use]
pub mod dispute_resolver;

impl PartialEq for Cell<i16> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq for Cell<char> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub struct Boxer {
    dispute_resolver: SimpleDisputeResolver<i16>,
    grid_converter: SimpleGridConverter<i16, char, DisassemblerError>,
}

// implement Default for Boxer
impl Default for Boxer {
    fn default() -> Self {
        let mut self_ = Boxer {
            dispute_resolver: SimpleDisputeResolver::new(),
            grid_converter: SimpleGridConverter::new(),
        };

        self_.grid_converter.add_rules(vec![
            Box::new(|value: i16| -> Result<char, DisassemblerError> {
                let chars = vec![' ', '─', '│', '┌', '└', '├', '┴', '┤', '┬', '┼'];
                if value < 0 || value >= chars.len() as i16 {
                    Err(DisassemblerError::new(0x0, format!("Invalid value: {}", value), DisassemblerErrorType::BoxerError))
                } else {
                    Ok(chars[value as usize])
                }
            }),
        ]);

        self_.dispute_resolver.add_rules(rules_unord!(
            (1, 3, 8), (1, 5, 9), (5, 2, 9),
            (2, 8, 9), (1, 7, 9), (1, 2, 9),
            (2, 3, 5), (2, 4, 5), (4, 1, 6),
            (0, 0, 0), (0, 1, 1), (0, 2, 2),
            (0, 3, 3), (0, 4, 4), (0, 5, 5),
            (0, 6, 6), (0, 7, 7), (0, 8, 8),
            (0, 9, 9)
        ));

        self_
    }
}

impl Boxer {
    pub fn new() -> Boxer {
        Self::default()
    }

    pub fn resolve_differences(&self, mut grid_left: Grid<i16>, mut grid_right: Grid<i16>) -> Grid<i16> {
        grid_left.add_empty_column_left(1);
        grid_right.stretch_right(grid_left.width-grid_right.width);
        let differences = grid_left.diff(grid_right);
        for difference in differences {
            let dispute = self.dispute_resolver.resolve_tuple((difference.0.value, difference.1.value));
            if let Some(dispute) = dispute {
                grid_left.set_cell(difference.0.x, difference.0.y, dispute);
            } else {
                print!("No dispute found for: ");
                println!("{}, {}", difference.0.value, difference.1.value);
            }
        }
        grid_left
    }

    pub fn convert_grid_to_lines(&self, grid: Grid<i16>) -> Result<Grid<char>, DisassemblerError> {
        let grid = self.grid_converter.convert(grid)?;
        Ok(grid)
    }

    pub fn grid_from_jump_data(&self, jump_data: (i16, Vec<i16>), max_jump: i32) -> Grid<i16> {
        // width of grid will always be 2
        // height will be max_jump
        let mut grid = Grid::new(2, max_jump, 0);
        let mut do_branch = |idx: i16| {
            if idx == jump_data.0 {
                grid.set_cell(1, idx as i32, 1);
                grid.set_cell(0, idx as i32, 4);
            } else if jump_data.1.contains(&idx) {
                if idx != *jump_data.1.first().unwrap() {
                    grid.set_cell(1, idx as i32, 1);
                    grid.set_cell(0, idx as i32, 5);
                } else {
                    grid.set_cell(1, idx as i32, 1);
                    grid.set_cell(0, idx as i32, 3);
                }
            } else {
                grid.set_cell(0, idx as i32, 2);
            }
        };

        do_branch(jump_data.0);
        for idx in *jump_data.1.first().unwrap()..jump_data.0 {
            do_branch(idx);
        }

        grid
    }
}