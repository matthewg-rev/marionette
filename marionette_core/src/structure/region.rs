// Purpose: defines *code* regions for general purpose disassemblies.
// src\structure\region.rs

pub enum OperandType {
    Register(Box<dyn Data>),
    Immediate(Box<dyn Immediate>),
}

pub enum ImmediateType {
    Byte(u8),
    Word(u16),
    Dword(u32),
    Qword(u64),
}

trait Immediate: Data {
    fn value(&self) -> ImmediateType;
}

trait Data {
    fn get_data(&self) -> &[u8];
    fn get_data_mut(&mut self) -> &mut [u8];
}

trait Operation: Data {
    fn name(&self) -> &str;
    fn opcode(&self) -> &[u8];
}

trait Operand: Data {
    fn value(&self) -> OperandType;
}

trait Code: Data {
    fn reference(&self) -> Box<dyn Code>;
    fn operation(&self) -> Box<dyn Operation>;
    fn operands(&self) -> Vec<Box<dyn Operand>>;
}