use marionette_core::{
    assembly::*,
    byte_stream::*
};

pub struct LuaHeader {
    pub raw: Vec<u8>,
    pub range: Range,

    pub signature: u32,
    pub version: u8,
    pub format: u8,
    pub endianness: u8,
    pub int_size: u8,
    pub size_t_size: u8,
    pub instruction_size: u8,
    pub lua_number_size: u8,
    pub integral_flag: u8
}

#[derive(Debug, PartialEq)]
pub enum LuaOpcode {
    // opcode A
    A(u8, u8),
    // opcode sBx
    SBx(u8, i16),
    // opcode A B
    AB(u8, u8, u8),
    // opcode A C
    AC(u8, u8, u8),
    // opcode A Bx
    ABx(u8, u8, u16),
    // opcode A sBx
    AsBx(u8, u8, i16),
    // opcode A B C
    ABC(u8, u8, u8, u8),
}

pub struct LuaInstruction {
    pub raw: Vec<u8>,
    pub range: Range,

    pub instruction: LuaOpcode,
}

#[derive(Debug, PartialEq)]
pub enum LuaConstantType {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

pub struct LuaConstant {
    pub raw: Vec<u8>,
    pub range: Range,

    pub constant: LuaConstantType,
}

pub struct LuaLocal {
    pub raw: Vec<u8>,
    pub range: Range,

    pub name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

pub struct LuaUpvalue {
    pub raw: Vec<u8>,
    pub range: Range,

    pub name: String,
}

pub struct LuaFunction {
    pub raw: Vec<u8>,
    pub range: Range,

    pub name: String,
    pub first_line: u64,
    pub last_line: u64,

    pub num_upvalues: u8,
    pub num_parameters: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,

    pub code_size: u64,
    pub code: Vec<LuaInstruction>,

    pub constant_size: u64,
    pub constants: Vec<LuaConstant>,

    pub function_size: u64,
    pub functions: Vec<LuaFunction>,

    pub line_info_size: u64,
    pub line_info: Vec<u32>,

    pub local_size: u64,
    pub locals: Vec<LuaLocal>,

    pub upvalue_size: u64,
    pub upvalues: Vec<LuaUpvalue>,
}

impl ByteStreamRead for LuaHeader {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(12) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaHeader".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let start = stream.caret();
        let mut raw = Vec::new();
        let signature = u32::read(stream)?;
        let version = u8::read(stream)?;
        let format = u8::read(stream)?;
        let endianness = u8::read(stream)?;
        let int_size = u8::read(stream)?;
        let size_t_size = u8::read(stream)?;
        let instruction_size = u8::read(stream)?;
        let lua_number_size = u8::read(stream)?;
        let integral_flag = u8::read(stream)?;
        let end = stream.caret();

        let sig_bytes = signature.to_le_bytes();
        raw.extend_from_slice(&sig_bytes);
        raw.extend_from_slice(
            &[version, format, endianness, int_size, size_t_size, instruction_size, lua_number_size, integral_flag]
        );

        stream.add_context(int_size);
        stream.add_context(size_t_size);
        stream.add_context(instruction_size);
        stream.add_context(lua_number_size);

        Ok(LuaHeader {
            raw: raw,
            range: Range::new(start as u64, end as u64),

            signature,
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

impl ByteStreamRead for LuaOpcode {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let instruction_size = context[2];
        // TODO: for future lua versions / different platforms
        // the instruction size may change
        
        if stream.is_out_of_bounds(instruction_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaOpcode".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let raw = u32::read(stream)?;
        let opcode = raw & 0x3F;
        let a = (raw >> 6) & 0xFF;
        let b = (raw >> 23) & 0x1FF;
        let c = (raw >> 14) & 0x1FF;
        let sbx = (raw >> 14) as i32 - 0x1FFFF;


    }
}