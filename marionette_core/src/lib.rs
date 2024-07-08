// Purpose: main library file for the disassembler
// src\lib.rs

pub mod lexer_service;
pub mod python_lexer;
pub mod general_lexer;

pub mod byte_stream;
pub mod mproj;

mod lib {}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::{byte_stream::{ByteStream, ByteStreamWrite, ByteStreamRead}, mproj::RawProject};
    use super::*;

    #[test]
    pub fn proj_read_write() {
        let mut stream = ByteStream::new(Vec::new());
        let proj = mproj::MarionetteProject::new();
        proj.write(&mut stream).unwrap();
        stream = ByteStream::from(&stream);
        let raw: RawProject = RawProject::read(&mut stream).unwrap();
        println!("{:?}", raw.project_version);
    }
}
