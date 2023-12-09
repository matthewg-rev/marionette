// Purpose: main library file for the disassembler
// src\lib.rs

pub mod disassembler;
pub mod byte_stream;
pub mod boxer;
pub mod plugin;
pub mod textualizer;
pub mod cfg;

mod lib {}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::boxer::grid::Grid;
    use crate::byte_stream::ByteStream;
    use crate::plugin::interface::{Function, PluginError};
    use crate::textualizer::Textualizer;
    use super::*;

    #[test]
    pub fn lua_disassemble() {
        let mut plugin_directory = plugin::PluginDirectory::new();
        unsafe {
            plugin_directory
                .load("C:\\Users\\MattG\\CLionProjects\\marionette\\marionette_lua\\target\\debug\\marionette_lua.dll")
                .expect("Couldn't load library");
        }

        // read console input from user
        let mut input_file = String::new();
        let mut dump_file = String::new();
        std::io::stdin().read_line(&mut input_file).expect("Failed to read line");
        std::io::stdin().read_line(&mut dump_file).expect("Failed to read line");

        let input_file = input_file.trim();
        let dump_file = dump_file.trim();

        let file_bytes: Vec<u8> = std::fs::read(input_file).expect("failed to read file");
        let (can_disassemble, disassembler) = plugin_directory.can_disassemble(&file_bytes);
        if can_disassemble {
            let disassembler = disassembler.unwrap();
            let disassembly_instance = disassembler
                .get_function("new_disassembly_instance").unwrap()
                .call(file_bytes).unwrap();

            // we will never actually touch the `instance` variable as it is a state for the plugin
            // for us to keep and pass whenever we invoke a function that depends on a state instance
            let disassembly_instance = disassembler
                .get_function("disassemble").unwrap()
                .call(disassembly_instance).unwrap();

            let functions = disassembler
                .get_function("get_functions").unwrap()
                .call(disassembly_instance).unwrap();
            let functions = ByteStream::new(functions).to_vec_u64();
            println!("functions: {:?}", functions);
        } else {
            println!("Cannot disassemble file");
        }
    }

    #[test]
    pub fn boxer_test() {
        let boxer = boxer::Boxer::new();
        let mut compiled_jump_lines = Grid::from_unsorted(2, 25, vec![], 0);
        let mut generated_jump_lines = Vec::new();
        for _ in 0..10 {
            let mut jump_line = Grid::from_unsorted(2, 25, vec![], 0);
            let jump_line_length = rand::random::<usize>() % 10 + 3;
            let jump_line_start = rand::random::<usize>() % (25 - jump_line_length);
            jump_line.set_cell(0, jump_line_start as i32, 3);
            jump_line.set_cell(1, jump_line_start as i32, 1);
            for i in 1..jump_line_length-2 {
                jump_line.set_cell(0, (jump_line_start + i) as i32, 2);
            }
            jump_line.set_cell(0, (jump_line_start + jump_line_length - 2) as i32, 4);
            jump_line.set_cell(1, (jump_line_start + jump_line_length - 2) as i32, 1);
            generated_jump_lines.push(jump_line.clone());
        }

        for jump_line in generated_jump_lines {
            compiled_jump_lines = boxer.resolve_differences(compiled_jump_lines, jump_line);
        }
        println!("{}", boxer.convert_grid_to_lines(compiled_jump_lines).unwrap().to_string());
    }
}
