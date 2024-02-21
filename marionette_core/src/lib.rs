// Purpose: main library file for the disassembler
// src\lib.rs

pub mod byte_stream;
pub mod mproj;

mod lib {}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::byte_stream::ByteStream;
    use super::*;

    #[test]
    pub fn lua_disassemble() {
        
    }

    #[test]
    pub fn boxer_test() {
        
    }
}
