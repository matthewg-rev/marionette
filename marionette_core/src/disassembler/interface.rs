// Purpose: A Generic Disassembly interface for the core library.
// Path: src\interface

pub struct Metadata {
    pub tag: String,
    pub value: String,
}

pub struct Instruction {
    pub metadata: Vec<Metadata>,
    pub start_address: u64,
    pub end_address: u64,

    pub mnemonic: String,
    pub operands: Vec<String>,
}

pub struct Function {
    pub metadata: Vec<Metadata>,
    pub start_address: u64,
    pub end_address: u64,
}

pub struct Disassembly {
    pub metadata: Vec<Metadata>,
    pub start_address: u64,

    pub functions: Vec<Function>,
    pub instructions: Vec<Instruction>,

    pub end_address: u64,
}