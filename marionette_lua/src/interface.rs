use std::fmt;
use std::fmt::{Debug, Display};
use bincode::{Decode, Encode};
use marionette_core::byte_stream::ByteStream;
use crate::configuration::Configuration;

macro_rules! implement_fmt_trait {
    ($trait_name:ident, $type_name:ident, |$self:ident, $f:ident| $fmt_code:block) => {
        impl $trait_name for $type_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let $self = self;
                let $f = f;
                $fmt_code
            }
        }
    };
}

#[derive(Clone)]
pub struct AddressedData<T: Encode + Decode + Clone + Display> {
    pub data: T,
    pub start_address: u64,
    pub end_address: u64
}

impl<T: Encode + Decode + Clone + Display> Encode for AddressedData<T> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.data, encoder)?;
        Encode::encode(&self.start_address, encoder)?;
        Encode::encode(&self.end_address, encoder)?;
        Ok(())
    }
}

impl<T: Encode + Decode + Clone + Display> Decode for AddressedData<T> {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let data = Decode::decode(decoder)?;
        let start_address = Decode::decode(decoder)?;
        let end_address = Decode::decode(decoder)?;
        Ok(AddressedData {
            data,
            start_address,
            end_address
        })
    }
}

impl<T: Encode + Decode + Clone + Display> Display for AddressedData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x} -> 0x{:x}: {}", self.start_address, self.end_address, self.data)
    }
}

impl<T: Encode + Decode + Clone + Display> AddressedData<T> {
    // a Fn as an argument would be:
    pub fn new(stream: &mut ByteStream, read: fn(&mut ByteStream) -> T) -> (AddressedData<T>, &mut ByteStream) {
        //let mut stream = ByteStream::new(bytes);
        let start_address = stream.current_address();
        stream.set_address(start_address);
        let data = read(stream);
        let end_address = stream.current_address();
        (AddressedData {
            data,
            start_address,
            end_address
        }, stream)
    }
}

impl<T: Encode + Decode + Clone + Default + Display> AddressedData<T> {
    pub fn blank() -> AddressedData<T> {
        AddressedData {
            data: T::default(),
            start_address: 0,
            end_address: 0
        }
    }
}

#[derive(Clone)]
pub struct LuaDisassemblerInstance {
    pub raw: Vec<u8>,

    pub upvalues: Vec<String>,
    pub allocated_upvalues: Vec<String>,

    pub configuration: Configuration,
    pub disassembly: LuaDisassembly,
}

#[derive(Clone)]
pub struct LuaDisassembly {
    pub header: AddressedData<LuaHeader>,
    pub entry_point: AddressedData<LuaChunk>,
    pub functions: Vec<AddressedData<LuaChunk>>,
}

#[derive(Clone)]
pub struct LuaHeader {
    pub magic: AddressedData<u32>,
    pub version: AddressedData<u8>,
    pub format: AddressedData<u8>,
    pub endianness: AddressedData<u8>,
    pub int_size: AddressedData<u8>,
    pub size_t_size: AddressedData<u8>,
    pub instruction_size: AddressedData<u8>,
    pub lua_number_size: AddressedData<u8>,
    pub integral_flag: AddressedData<u8>
}

#[derive(Clone)]
pub struct LuaChunkHeader {
    pub name: AddressedData<String>,
    pub line_defined: AddressedData<u64>,
    pub last_line_defined: AddressedData<u64>,
    pub num_upvalues: AddressedData<u8>,
    pub num_parameters: AddressedData<u8>,
    pub is_vararg: AddressedData<u8>,
    pub max_stack_size: AddressedData<u8>,
}

#[derive(Clone)]
pub struct LuaChunk {
    pub header: AddressedData<LuaChunkHeader>,
    pub instructions: Vec<AddressedData<LuaInstruction>>,
    pub constants: Vec<AddressedData<LuaConstant>>,
    pub functions: Vec<AddressedData<LuaChunk>>,
    pub line_info: Vec<AddressedData<u32>>,
    pub locals: Vec<AddressedData<LuaLocal>>,
    pub upvalues: Vec<AddressedData<LuaUpvalue>>,
}

#[derive(Clone, Default)]
pub struct LuaInstruction {
    pub opcode: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub bx: u16,
    pub sbx: u16,
}

#[derive(Clone)]
pub enum LuaConstantValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Clone)]
pub struct LuaConstant {
    pub value: AddressedData<LuaConstantValue>,
    pub type_tag: AddressedData<u8>,
}

#[derive(Clone)]
pub struct LuaLocal {
    pub name: AddressedData<String>,
    pub start_pc: AddressedData<u32>,
    pub end_pc: AddressedData<u32>,
}

#[derive(Clone)]
pub struct LuaUpvalue {
    pub name: AddressedData<String>,
}

impl Encode for LuaHeader {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.magic, encoder)?;
        Encode::encode(&self.version, encoder)?;
        Encode::encode(&self.format, encoder)?;
        Encode::encode(&self.endianness, encoder)?;
        Encode::encode(&self.int_size, encoder)?;
        Encode::encode(&self.size_t_size, encoder)?;
        Encode::encode(&self.instruction_size, encoder)?;
        Encode::encode(&self.lua_number_size, encoder)?;
        Encode::encode(&self.integral_flag, encoder)?;
        Ok(())
    }
}

impl Encode for LuaDisassemblerInstance {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.raw, encoder)?;
        Encode::encode(&self.upvalues, encoder)?;
        Encode::encode(&self.allocated_upvalues, encoder)?;
        Encode::encode(&self.configuration, encoder)?;
        Encode::encode(&self.disassembly, encoder)?;
        Ok(())
    }
}

impl Decode for LuaDisassemblerInstance {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let raw = Decode::decode(decoder)?;
        let upvalues = Decode::decode(decoder)?;
        let allocated_upvalues = Decode::decode(decoder)?;
        let configuration = Decode::decode(decoder)?;
        let disassembly = Decode::decode(decoder)?;
        Ok(LuaDisassemblerInstance {
            raw,
            upvalues,
            allocated_upvalues,
            configuration,
            disassembly
        })
    }
}

impl Encode for LuaDisassembly {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.header, encoder)?;
        Encode::encode(&self.entry_point, encoder)?;
        Encode::encode(&self.functions, encoder)?;
        Ok(())
    }
}

impl Decode for LuaDisassembly {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let header = Decode::decode(decoder)?;
        let entry_point = Decode::decode(decoder)?;
        let functions = Decode::decode(decoder)?;
        Ok(LuaDisassembly {
            header,
            entry_point,
            functions
        })
    }
}

impl Decode for LuaHeader {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let magic = Decode::decode(decoder)?;
        let version = Decode::decode(decoder)?;
        let format = Decode::decode(decoder)?;
        let endianness = Decode::decode(decoder)?;
        let int_size = Decode::decode(decoder)?;
        let size_t_size = Decode::decode(decoder)?;
        let instruction_size = Decode::decode(decoder)?;
        let lua_number_size = Decode::decode(decoder)?;
        let integral_flag = Decode::decode(decoder)?;
        Ok(LuaHeader {
            magic,
            version,
            format,
            endianness,
            int_size,
            size_t_size,
            instruction_size,
            lua_number_size,
            integral_flag
        })
    }
}

impl Encode for LuaChunkHeader {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.name, encoder)?;
        Encode::encode(&self.line_defined, encoder)?;
        Encode::encode(&self.last_line_defined, encoder)?;
        Encode::encode(&self.num_upvalues, encoder)?;
        Encode::encode(&self.num_parameters, encoder)?;
        Encode::encode(&self.is_vararg, encoder)?;
        Encode::encode(&self.max_stack_size, encoder)?;
        Ok(())
    }
}

impl Decode for LuaChunkHeader {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let name = Decode::decode(decoder)?;
        let line_defined = Decode::decode(decoder)?;
        let last_line_defined = Decode::decode(decoder)?;
        let num_upvalues = Decode::decode(decoder)?;
        let num_parameters = Decode::decode(decoder)?;
        let is_vararg = Decode::decode(decoder)?;
        let max_stack_size = Decode::decode(decoder)?;
        Ok(LuaChunkHeader {
            name,
            line_defined,
            last_line_defined,
            num_upvalues,
            num_parameters,
            is_vararg,
            max_stack_size
        })
    }
}

impl Encode for LuaChunk {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.header, encoder)?;
        Encode::encode(&self.instructions, encoder)?;
        Encode::encode(&self.constants, encoder)?;
        Encode::encode(&self.functions, encoder)?;
        Encode::encode(&self.line_info, encoder)?;
        Encode::encode(&self.locals, encoder)?;
        Encode::encode(&self.upvalues, encoder)?;
        Ok(())
    }
}

impl Decode for LuaChunk {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let header = Decode::decode(decoder)?;
        let instructions = Decode::decode(decoder)?;
        let constants = Decode::decode(decoder)?;
        let functions = Decode::decode(decoder)?;
        let line_info = Decode::decode(decoder)?;
        let locals = Decode::decode(decoder)?;
        let upvalues = Decode::decode(decoder)?;
        Ok(LuaChunk {
            header,
            instructions,
            constants,
            functions,
            line_info,
            locals,
            upvalues
        })
    }
}

impl Encode for LuaInstruction {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.opcode, encoder)?;
        Encode::encode(&self.a, encoder)?;
        Encode::encode(&self.b, encoder)?;
        Encode::encode(&self.c, encoder)?;
        Encode::encode(&self.bx, encoder)?;
        Encode::encode(&self.sbx, encoder)?;
        Ok(())
    }
}

impl Decode for LuaInstruction {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let opcode = Decode::decode(decoder)?;
        let a = Decode::decode(decoder)?;
        let b = Decode::decode(decoder)?;
        let c = Decode::decode(decoder)?;
        let bx = Decode::decode(decoder)?;
        let sbx = Decode::decode(decoder)?;
        Ok(LuaInstruction {
            opcode,
            a,
            b,
            c,
            bx,
            sbx
        })
    }
}

impl Encode for LuaConstantValue {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            LuaConstantValue::Nil => {
                Encode::encode(&0, encoder)?;
            },
            LuaConstantValue::Boolean(b) => {
                Encode::encode(&1, encoder)?;
                Encode::encode(b, encoder)?;
            },
            LuaConstantValue::Number(n) => {
                Encode::encode(&3, encoder)?;
                Encode::encode(n, encoder)?;
            },
            LuaConstantValue::String(s) => {
                Encode::encode(&4, encoder)?;
                Encode::encode(s, encoder)?;
            }
        }
        Ok(())
    }
}

impl Decode for LuaConstantValue {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let type_id = Decode::decode(decoder)?;
        match type_id {
            0 => Ok(LuaConstantValue::Nil),
            1 => {
                let b = Decode::decode(decoder)?;
                Ok(LuaConstantValue::Boolean(b))
            },
            3 => {
                let n = Decode::decode(decoder)?;
                Ok(LuaConstantValue::Number(n))
            },
            4 => {
                let s = Decode::decode(decoder)?;
                Ok(LuaConstantValue::String(s))
            },
            _ => Err(bincode::error::DecodeError::Other("Invalid constant type"))
        }
    }
}

impl Encode for LuaConstant {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.type_tag, encoder)?;
        Encode::encode(&self.value, encoder)?;
        Ok(())
    }
}

impl Decode for LuaConstant {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let type_tag = Decode::decode(decoder)?;
        let value = Decode::decode(decoder)?;
        Ok(LuaConstant {
            type_tag,
            value
        })
    }
}

impl Encode for LuaLocal {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.name, encoder)?;
        Encode::encode(&self.start_pc, encoder)?;
        Encode::encode(&self.end_pc, encoder)?;
        Ok(())
    }
}

impl Decode for LuaLocal {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let name = Decode::decode(decoder)?;
        let start_pc = Decode::decode(decoder)?;
        let end_pc = Decode::decode(decoder)?;
        Ok(LuaLocal {
            name,
            start_pc,
            end_pc
        })
    }
}

impl Encode for LuaUpvalue {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.name, encoder)?;
        Ok(())
    }
}

impl Decode for LuaUpvalue {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let name = Decode::decode(decoder)?;
        Ok(LuaUpvalue {
            name
        })
    }
}

impl Default for LuaDisassembly {
    fn default() -> LuaDisassembly {
        LuaDisassembly {
            header: AddressedData::blank(),
            entry_point: AddressedData::blank(),
            functions: Vec::new(),
        }
    }
}

impl Default for LuaHeader {
    fn default() -> LuaHeader {
        LuaHeader {
            magic: AddressedData::blank(),
            version: AddressedData::blank(),
            format: AddressedData::blank(),
            endianness: AddressedData::blank(),
            int_size: AddressedData::blank(),
            size_t_size: AddressedData::blank(),
            instruction_size: AddressedData::blank(),
            lua_number_size: AddressedData::blank(),
            integral_flag: AddressedData::blank()
        }
    }
}

impl Default for LuaChunkHeader {
    fn default() -> LuaChunkHeader {
        LuaChunkHeader {
            name: AddressedData { start_address: 0, end_address: 0, data: String::from("") },
            line_defined: AddressedData::blank(),
            last_line_defined: AddressedData::blank(),
            num_upvalues: AddressedData::blank(),
            num_parameters: AddressedData::blank(),
            is_vararg: AddressedData::blank(),
            max_stack_size: AddressedData::blank()
        }
    }
}

impl Default for LuaChunk {
    fn default() -> LuaChunk {
        LuaChunk {
            header: AddressedData::blank(),
            instructions: Vec::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            line_info: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new()
        }
    }
}

impl Display for LuaHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LuaHeader:\n");
        write!(f, "\t{}\n", self.magic);
        write!(f, "\t{}\n", self.version);
        write!(f, "\t{}\n", self.format);
        write!(f, "\t{}\n", self.endianness);
        write!(f, "\t{}\n", self.int_size);
        write!(f, "\t{}\n", self.size_t_size);
        write!(f, "\t{}\n", self.instruction_size);
        write!(f, "\t{}\n", self.lua_number_size);
        write!(f, "\t{}\n", self.integral_flag)
    }
}

implement_fmt_trait!(Display, LuaChunkHeader, |header, formatter| {
    write!(formatter, "🪨 {}:{}\n\t{} -> {}\n\t{} up_values\n\t{} parameters\n\t{} stack size\n\t{}",
        header.name.data,
        header.line_defined.data,
        header.line_defined.data,
        header.last_line_defined.data,
        header.num_upvalues.data,
        header.num_parameters.data,
        header.max_stack_size.data,
        if header.is_vararg.data == 1 { "vararg" } else { "not vararg" }
    )
});
implement_fmt_trait!(Debug, LuaChunkHeader, |header, formatter| {
    formatter.debug_struct("LuaChunkHeader")
        .field("name", &header.name)
        .field("line_defined", &header.line_defined.data)
        .field("last_line_defined", &header.last_line_defined.data)
        .field("num_upvalues", &header.num_upvalues.data)
        .field("num_parameters", &header.num_parameters.data)
        .field("is_vararg", &header.is_vararg.data)
        .field("max_stack_size", &header.max_stack_size.data)
        .finish()
});

implement_fmt_trait!(Display, LuaChunk, |chunk, formatter| {
    write!(formatter, "🪨 {}:{}", chunk.header.data.name.data, chunk.header.data.line_defined.data)
});
implement_fmt_trait!(Debug, LuaChunk, |chunk, formatter| {
    formatter.debug_struct("LuaChunk")
        .field("header", &chunk.header.data)
        .field("instructions", &chunk.instructions)
        .finish()
});

implement_fmt_trait!(Display, LuaInstruction, |instruction, formatter| {
    write!(formatter, "{} {} {} {} {} {}", instruction.opcode, instruction.a, instruction.b, instruction.c, instruction.bx, instruction.sbx)
});
implement_fmt_trait!(Debug, LuaInstruction, |instruction, formatter| {
    formatter.debug_struct("LuaInstruction")
        .field("opcode", &instruction.opcode)
        .field("a", &instruction.a)
        .field("b", &instruction.b)
        .field("c", &instruction.c)
        .field("bx", &instruction.bx)
        .field("sbx", &instruction.sbx)
        .finish()
});

implement_fmt_trait!(Display, LuaConstantValue, |constant_value, formatter| {
    match constant_value {
        LuaConstantValue::Nil => write!(formatter, "nil"),
        LuaConstantValue::Boolean(b) => write!(formatter, "{}", b),
        LuaConstantValue::Number(n) => write!(formatter, "{}", n),
        LuaConstantValue::String(s) => write!(formatter, "\"{}\"", s)
    }
});
implement_fmt_trait!(Debug, LuaConstantValue, |constant_value, formatter| {
    formatter.debug_struct("LuaConstantValue")
        .field("data", &constant_value)
        .finish()
});

implement_fmt_trait!(Display, LuaConstant, |constant, formatter| { write!(formatter, "{}", constant.value) });
implement_fmt_trait!(Debug, LuaConstant, |constant, formatter| {
    formatter.debug_struct("LuaConstant")
        .field("value", &constant.value.data)
        .field("type_tag", &constant.type_tag.data)
        .finish()
});

implement_fmt_trait!(Display, LuaLocal, |local, formatter| { write!(formatter, "{}", local.name) });
implement_fmt_trait!(Debug, LuaLocal, |local, formatter| {
    formatter.debug_struct("LuaLocal")
        .field("name", &local.name)
        .field("start_pc", &local.start_pc.data)
        .field("end_pc", &local.end_pc.data)
        .finish()
});

implement_fmt_trait!(Display, LuaUpvalue, |upvalue, formatter| { write!(formatter, "{}", upvalue.name) });
implement_fmt_trait!(Debug, LuaUpvalue, |upvalue, formatter| {
    formatter.debug_struct("LuaUpvalue")
        .field("name", &upvalue.name)
        .finish()
});

type AddressedString = AddressedData<String>;
implement_fmt_trait!(Debug, AddressedString, |addressed_data, formatter| {
    formatter.debug_struct("AddressedData")
        .field("start_address", &addressed_data.start_address)
        .field("end_address", &addressed_data.end_address)
        .field("data", &addressed_data.data)
        .finish()
});

type AddressedInstruction = AddressedData<LuaInstruction>;
implement_fmt_trait!(Debug, AddressedInstruction, |addressed_data, formatter| {
    formatter.debug_struct("AddressedData")
        .field("start_address", &addressed_data.start_address)
        .field("end_address", &addressed_data.end_address)
        .field("data", &addressed_data.data)
        .finish()
});

type AddressedConstant = AddressedData<LuaConstant>;
implement_fmt_trait!(Debug, AddressedConstant, |addressed_data, formatter| {
    formatter.debug_struct("AddressedData")
        .field("start_address", &addressed_data.start_address)
        .field("end_address", &addressed_data.end_address)
        .field("data", &addressed_data.data)
        .finish()
});