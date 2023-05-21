// Purpose: handles disassembly requests for marionette
// Path: src\disassembler.rs

pub struct Disassembler {
    bytes: Vec<u8>,
}

impl Disassembler {
    pub fn new(bytes: Vec<u8>) -> Self {
        // create a new disassembler
        Disassembler {
            bytes
        }
    }
}