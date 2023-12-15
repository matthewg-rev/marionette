// Purpose: defines *code* regions for general purpose disassemblies.
// src\structure\region.rs

trait Data {
    fn get_data(&self) -> &[u8];
    fn get_data_mut(&mut self) -> &mut [u8];
}

trait Operation: Data {
    fn name(&self) -> &str;
    fn opcode(&self) -> &[u8];
}

trait Code: Data {
    fn reference(&self) -> Box<dyn Code>;
    fn operation(&self) -> Box<dyn Operation>;
}