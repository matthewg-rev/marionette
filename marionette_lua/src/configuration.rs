use bincode::{Decode, Encode};

#[derive(Clone)]
pub struct Configuration {
    pub version: u8,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct Reg {
    pub name: String,
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub name: String,
    pub registers: Vec<Reg>
}

impl Configuration {
    pub fn new(version: u8, instructions: Vec<Instruction>) -> Self {
        Configuration { version, instructions }
    }
}

impl Reg {
    pub fn new(name: &str) -> Self {
        Reg { name: name.to_string() }
    }
}

impl Instruction {
    pub fn new(opcode: u8, name: &str, registers: Vec<Reg>) -> Self {
        Instruction { opcode, name: name.to_string(), registers }
    }
}

impl Encode for Configuration {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.version, encoder)?;
        Encode::encode(&self.instructions, encoder)?;
        Ok(())
    }
}

impl Decode for Configuration {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let version = Decode::decode(decoder)?;
        let instructions = Decode::decode(decoder)?;
        Ok(Configuration::new(version, instructions))
    }
}

impl Encode for Reg {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.name.to_string(), encoder)?;
        Ok(())
    }
}

impl Decode for Reg {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let name: String = Decode::decode(decoder)?;
        Ok(Reg::new(name.as_str()))
    }
}

impl Encode for Instruction {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.opcode, encoder)?;
        Encode::encode(&self.name.to_string(), encoder)?;
        Encode::encode(&self.registers, encoder)?;
        Ok(())
    }
}

impl Decode for Instruction {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let opcode = Decode::decode(decoder)?;
        let name: String = Decode::decode(decoder)?;
        let registers = Decode::decode(decoder)?;
        Ok(Instruction::new(opcode, name.as_str(), registers))
    }
}