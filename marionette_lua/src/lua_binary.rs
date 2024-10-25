use lazy_static::*;
use std::{collections::HashMap, fmt::Debug};
use marionette_core::{
    assembly::*,
    byte_stream::*
};

// TODO: Clean up this file
// The main issue at the moment with this file the amount of times we spend checking the header
// configuration to see what size of the variable we need to read.
// We need to make functions that do this for us, so we don't have to repeat the same code over and over again.

lazy_static! {
    static ref OPCODE_LAYOUT: HashMap<LuaOpcode, LuaLayout> = vec![
        (LuaOpcode::MOVE, LuaLayout::AB(LuaOpcode::MOVE, 0, 0)),        // OP_MOVE
        (LuaOpcode::LOADK, LuaLayout::ABx(LuaOpcode::LOADK, 0, 0)),       // OP_LOADK
        (LuaOpcode::LOADBOOL, LuaLayout::ABC(LuaOpcode::LOADBOOL, 0, 0, 0)),    // OP_LOADBOOL
        (LuaOpcode::LOADNIL, LuaLayout::AB(LuaOpcode::LOADNIL, 0, 0)),        // OP_LOADNIL
        (LuaOpcode::GETUPVAL, LuaLayout::AB(LuaOpcode::GETUPVAL, 0, 0)),        // OP_GETUPVAL

        (LuaOpcode::GETGLOBAL, LuaLayout::ABx(LuaOpcode::GETGLOBAL, 0, 0)),       // OP_GETGLOBAL
        (LuaOpcode::GETTABLE, LuaLayout::ABC(LuaOpcode::GETTABLE, 0, 0, 0)),    // OP_GETTABLE

        (LuaOpcode::SETGLOBAL, LuaLayout::ABx(LuaOpcode::SETGLOBAL, 0, 0)),       // OP_SETGLOBAL
        (LuaOpcode::SETUPVAL, LuaLayout::AB(LuaOpcode::SETUPVAL, 0, 0)),        // OP_SETUPVAL
        (LuaOpcode::SETTABLE, LuaLayout::ABC(LuaOpcode::SETTABLE, 0, 0, 0)),    // OP_SETTABLE

        (LuaOpcode::NEWTABLE, LuaLayout::ABC(LuaOpcode::NEWTABLE, 0, 0, 0)),   // OP_NEWTABLE

        (LuaOpcode::SELF, LuaLayout::ABC(LuaOpcode::SELF, 0, 0, 0)),   // OP_SELF

        (LuaOpcode::ADD, LuaLayout::ABC(LuaOpcode::ADD, 0, 0, 0)),   // OP_ADD
        (LuaOpcode::SUB, LuaLayout::ABC(LuaOpcode::SUB, 0, 0, 0)),   // OP_SUB
        (LuaOpcode::MUL, LuaLayout::ABC(LuaOpcode::MUL, 0, 0, 0)),   // OP_MUL
        (LuaOpcode::DIV, LuaLayout::ABC(LuaOpcode::DIV, 0, 0, 0)),   // OP_DIV
        (LuaOpcode::MOD, LuaLayout::ABC(LuaOpcode::MOD, 0, 0, 0)),   // OP_MOD
        (LuaOpcode::POW, LuaLayout::ABC(LuaOpcode::POW, 0, 0, 0)),   // OP_POW
        (LuaOpcode::UNM, LuaLayout::AB(LuaOpcode::UNM, 0, 0)),       // OP_UNM
        (LuaOpcode::NOT, LuaLayout::AB(LuaOpcode::NOT, 0, 0)),       // OP_NOT
        (LuaOpcode::LEN, LuaLayout::AB(LuaOpcode::LEN, 0, 0)),       // OP_LEN

        (LuaOpcode::CONCAT, LuaLayout::ABC(LuaOpcode::CONCAT, 0, 0, 0)),   // OP_CONCAT

        (LuaOpcode::JMP, LuaLayout::SBx(LuaOpcode::JMP, 0)),         // OP_JMP

        (LuaOpcode::EQ, LuaLayout::ABC(LuaOpcode::EQ, 0, 0, 0)),   // OP_EQ
        (LuaOpcode::LT, LuaLayout::ABC(LuaOpcode::LT, 0, 0, 0)),   // OP_LT
        (LuaOpcode::LE, LuaLayout::ABC(LuaOpcode::LE, 0, 0, 0)),   // OP_LE

        (LuaOpcode::TEST, LuaLayout::AC(LuaOpcode::TEST, 0, 0)),       // OP_TEST
        (LuaOpcode::TESTSET, LuaLayout::ABC(LuaOpcode::TESTSET, 0, 0, 0)),   // OP_TESTSET

        (LuaOpcode::CALL, LuaLayout::ABC(LuaOpcode::CALL, 0, 0, 0)),   // OP_CALL
        (LuaOpcode::TAILCALL, LuaLayout::ABC(LuaOpcode::TAILCALL, 0, 0, 0)),   // OP_TAILCALL
        (LuaOpcode::RETURN, LuaLayout::AB(LuaOpcode::RETURN, 0, 0)),       // OP_RETURN

        (LuaOpcode::FORLOOP, LuaLayout::AsBx(LuaOpcode::FORLOOP, 0, 0)),     // OP_FORLOOP
        (LuaOpcode::FORPREP, LuaLayout::AsBx(LuaOpcode::FORPREP, 0, 0)),     // OP_FORPREP

        (LuaOpcode::TFORLOOP, LuaLayout::AC(LuaOpcode::TFORLOOP, 0, 0)),       // OP_TFORLOOP

        (LuaOpcode::SETLIST, LuaLayout::ABC(LuaOpcode::SETLIST, 0, 0, 0)),   // OP_SETLIST

        (LuaOpcode::CLOSE, LuaLayout::A(LuaOpcode::CLOSE, 0)),           // OP_CLOSE
        (LuaOpcode::CLOSURE, LuaLayout::ABx(LuaOpcode::CLOSURE, 0, 0)),      // OP_CLOSURE

        (LuaOpcode::VARARG, LuaLayout::AB(LuaOpcode::VARARG, 0, 0)),       // OP_VARARG
    ].iter().copied().collect();
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum LuaOpcode {
    MOVE = 0,
    LOADK = 1,
    LOADBOOL = 2,
    LOADNIL = 3,
    GETUPVAL = 4,

    GETGLOBAL = 5,
    GETTABLE = 6,

    SETGLOBAL = 7,
    SETUPVAL = 8,
    SETTABLE = 9,

    NEWTABLE = 10,

    SELF = 11,

    ADD = 12,
    SUB = 13,
    MUL = 14,
    DIV = 15,
    MOD = 16,
    POW = 17,
    UNM = 18,
    NOT = 19,
    LEN = 20,

    CONCAT = 21,

    JMP = 22,

    EQ = 23,
    LT = 24,
    LE = 25,

    TEST = 26,
    TESTSET = 27,

    CALL = 28,
    TAILCALL = 29,
    RETURN = 30,

    FORLOOP = 31,
    FORPREP = 32,

    TFORLOOP = 33,

    SETLIST = 34,

    CLOSE = 35,
    CLOSURE = 36,

    VARARG = 37
}

impl ToString for LuaOpcode {
    fn to_string(&self) -> String {
        match self {
            LuaOpcode::MOVE => "MOVE",
            LuaOpcode::LOADK => "LOADK",
            LuaOpcode::LOADBOOL => "LOADBOOL",
            LuaOpcode::LOADNIL => "LOADNIL",
            LuaOpcode::GETUPVAL => "GETUPVAL",

            LuaOpcode::GETGLOBAL => "GETGLOBAL",
            LuaOpcode::GETTABLE => "GETTABLE",

            LuaOpcode::SETGLOBAL => "SETGLOBAL",
            LuaOpcode::SETUPVAL => "SETUPVAL",
            LuaOpcode::SETTABLE => "SETTABLE",

            LuaOpcode::NEWTABLE => "NEWTABLE",

            LuaOpcode::SELF => "SELF",

            LuaOpcode::ADD => "ADD",
            LuaOpcode::SUB => "SUB",
            LuaOpcode::MUL => "MUL",
            LuaOpcode::DIV => "DIV",
            LuaOpcode::MOD => "MOD",
            LuaOpcode::POW => "POW",
            LuaOpcode::UNM => "UNM",
            LuaOpcode::NOT => "NOT",
            LuaOpcode::LEN => "LEN",

            LuaOpcode::CONCAT => "CONCAT",

            LuaOpcode::JMP => "JMP",

            LuaOpcode::EQ => "EQ",
            LuaOpcode::LT => "LT",
            LuaOpcode::LE => "LE",

            LuaOpcode::TEST => "TEST",
            LuaOpcode::TESTSET => "TESTSET",

            LuaOpcode::CALL => "CALL",
            LuaOpcode::TAILCALL => "TAILCALL",
            LuaOpcode::RETURN => "RETURN",

            LuaOpcode::FORLOOP => "FORLOOP",
            LuaOpcode::FORPREP => "FORPREP",

            LuaOpcode::TFORLOOP => "TFORLOOP",

            LuaOpcode::SETLIST => "SETLIST",

            LuaOpcode::CLOSE => "CLOSE",
            LuaOpcode::CLOSURE => "CLOSURE",

            LuaOpcode::VARARG => "VARARG",
        }.to_string()
    }
}

impl TryFrom<String> for LuaOpcode {
    type Error = ();
    fn try_from(value: String) -> Result<Self, ()> {
        match value.as_str() {
            "MOVE" => Ok(LuaOpcode::MOVE),
            "LOADK" => Ok(LuaOpcode::LOADK),
            "LOADBOOL" => Ok(LuaOpcode::LOADBOOL),
            "LOADNIL" => Ok(LuaOpcode::LOADNIL),
            "GETUPVAL" => Ok(LuaOpcode::GETUPVAL),

            "GETGLOBAL" => Ok(LuaOpcode::GETGLOBAL),
            "GETTABLE" => Ok(LuaOpcode::GETTABLE),

            "SETGLOBAL" => Ok(LuaOpcode::SETGLOBAL),
            "SETUPVAL" => Ok(LuaOpcode::SETUPVAL),
            "SETTABLE" => Ok(LuaOpcode::SETTABLE),

            "NEWTABLE" => Ok(LuaOpcode::NEWTABLE),

            "SELF" => Ok(LuaOpcode::SELF),

            "ADD" => Ok(LuaOpcode::ADD),
            "SUB" => Ok(LuaOpcode::SUB),
            "MUL" => Ok(LuaOpcode::MUL),
            "DIV" => Ok(LuaOpcode::DIV),
            "MOD" => Ok(LuaOpcode::MOD),
            "POW" => Ok(LuaOpcode::POW),
            "UNM" => Ok(LuaOpcode::UNM),
            "NOT" => Ok(LuaOpcode::NOT),
            "LEN" => Ok(LuaOpcode::LEN),

            "CONCAT" => Ok(LuaOpcode::CONCAT),

            "JMP" => Ok(LuaOpcode::JMP),

            "EQ" => Ok(LuaOpcode::EQ),
            "LT" => Ok(LuaOpcode::LT),
            "LE" => Ok(LuaOpcode::LE),

            "TEST" => Ok(LuaOpcode::TEST),
            "TESTSET" => Ok(LuaOpcode::TESTSET),

            "CALL" => Ok(LuaOpcode::CALL),
            "TAILCALL" => Ok(LuaOpcode::TAILCALL),
            "RETURN" => Ok(LuaOpcode::RETURN),

            "FORLOOP" => Ok(LuaOpcode::FORLOOP),
            "FORPREP" => Ok(LuaOpcode::FORPREP),

            "TFORLOOP" => Ok(LuaOpcode::TFORLOOP),

            "SETLIST" => Ok(LuaOpcode::SETLIST),

            "CLOSE" => Ok(LuaOpcode::CLOSE),
            "CLOSURE" => Ok(LuaOpcode::CLOSURE),

            "VARARG" => Ok(LuaOpcode::VARARG),
            _ => Err(()),
        }
    }
}

impl From<u8> for LuaOpcode {
    fn from(value: u8) -> Self {
        match value {
            0 => LuaOpcode::MOVE,
            1 => LuaOpcode::LOADK,
            2 => LuaOpcode::LOADBOOL,
            3 => LuaOpcode::LOADNIL,
            4 => LuaOpcode::GETUPVAL,

            5 => LuaOpcode::GETGLOBAL,
            6 => LuaOpcode::GETTABLE,

            7 => LuaOpcode::SETGLOBAL,
            8 => LuaOpcode::SETUPVAL,
            9 => LuaOpcode::SETTABLE,

            10 => LuaOpcode::NEWTABLE,

            11 => LuaOpcode::SELF,

            12 => LuaOpcode::ADD,
            13 => LuaOpcode::SUB,
            14 => LuaOpcode::MUL,
            15 => LuaOpcode::DIV,
            16 => LuaOpcode::MOD,
            17 => LuaOpcode::POW,
            18 => LuaOpcode::UNM,
            19 => LuaOpcode::NOT,
            20 => LuaOpcode::LEN,

            21 => LuaOpcode::CONCAT,

            22 => LuaOpcode::JMP,

            23 => LuaOpcode::EQ,
            24 => LuaOpcode::LT,
            25 => LuaOpcode::LE,

            26 => LuaOpcode::TEST,
            27 => LuaOpcode::TESTSET,

            28 => LuaOpcode::CALL,
            29 => LuaOpcode::TAILCALL,
            30 => LuaOpcode::RETURN,

            31 => LuaOpcode::FORLOOP,
            32 => LuaOpcode::FORPREP,

            33 => LuaOpcode::TFORLOOP,

            34 => LuaOpcode::SETLIST,

            35 => LuaOpcode::CLOSE,
            36 => LuaOpcode::CLOSURE,

            37 => LuaOpcode::VARARG,
            _ => panic!("unknown opcode: {}", value)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LuaLayout {
    // opcode A
    A(LuaOpcode, u8),
    // opcode sBx
    SBx(LuaOpcode, i32),
    // opcode A B
    AB(LuaOpcode, u8, u16),
    // opcode A C
    AC(LuaOpcode, u8, u16),
    // opcode A Bx
    ABx(LuaOpcode, u8, u32),
    // opcode A sBx
    AsBx(LuaOpcode, u8, i32),
    // opcode A B C
    ABC(LuaOpcode, u8, u16, u16),
}

#[derive(PartialEq, Clone)]
pub struct LuaInstruction {
    pub raw: Vec<u8>,
    pub range: Range,

    pub opcode: LuaOpcode,
    pub components: LuaLayout,
    pub pc: u64,

    pub jump_target: Option<usize>,
}

impl Debug for LuaInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.opcode.to_string();
        write!(f, "{} ", self.pc)?;
        match &self.components {
            LuaLayout::A(_, a) => {
                write!(f, "{} {}", name, a)?;
            },
            LuaLayout::SBx(_, sbx) => {
                write!(f, "{} {} ", name, sbx)?;
            },
            LuaLayout::AB(_, a, b) => {
                write!(f, "{} {} {} ", name, a, b)?;
            },
            LuaLayout::AC(_, a, c) => {
                write!(f, "{} {} {} ", name, a, c)?;
            },
            LuaLayout::ABx(_, a, bx) => {
                write!(f, "{} {} {} ", name, a, bx)?;
            },
            LuaLayout::AsBx(_, a, sbx) => {
                write!(f, "{} {} {} ", name, a, sbx)?;
            },
            LuaLayout::ABC(_, a, b, c) => {
                write!(f, "{} {} {} {} ", name, a, b, c)?;
            },
        }

        if let Some(target) = self.jump_target {
            write!(f, "=> {}", target)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LuaConstantType {
    Nil(Vec<u8>),
    Boolean(Vec<u8>, bool),
    Number(Vec<u8>, f64),
    String(Vec<u8>, String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LuaConstant {
    pub raw: Vec<u8>,
    pub range: Range,

    pub constant: LuaConstantType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LuaLocal {
    pub raw: Vec<u8>,
    pub range: Range,

    pub name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LuaUpvalue {
    pub raw: Vec<u8>,
    pub range: Range,

    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
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

impl LuaFunction {
    pub fn find_instruction(&mut self, index: usize) -> Option<&LuaInstruction> {
        for (i, instruction) in self.code.iter().enumerate() {
            if i == index {
                return Some(instruction);
            }
        }

        None
    }

    pub fn update_targets(&mut self) {
        let code_len = self.code.len();
        for (index, instruction) in self.code.iter_mut().enumerate() {
            let opcode = instruction.opcode;

            match opcode {
                LuaOpcode::LOADBOOL => {
                    let c = match instruction.components {
                        LuaLayout::ABC(_, _, _, c) => c,
                        _ => 0
                    };

                    if c == 0 || index + 2 >= code_len {
                        continue;
                    }

                    instruction.jump_target = Some(index + 2);
                },
                LuaOpcode::EQ | LuaOpcode::LT | LuaOpcode::LE | LuaOpcode::TEST | LuaOpcode::TESTSET | LuaOpcode::TFORLOOP => {
                    if index + 2 >= code_len {
                        continue;
                    }

                    instruction.jump_target = Some(index + 2);
                },
                LuaOpcode::JMP | LuaOpcode::FORLOOP | LuaOpcode::FORPREP => {
                    let s_bx = match instruction.components {
                        LuaLayout::AsBx(_, _, s_bx) => s_bx,
                        LuaLayout::SBx(_, s_bx) => s_bx,
                        _ => 0
                    };

                    
                    let desired_pc = ((index + 1) as i32) + s_bx;
                    if desired_pc as usize >= code_len {
                        continue;
                    }

                    instruction.jump_target = Some(desired_pc as usize);
                }
                _ => continue,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LuaBinary {
    pub raw: Vec<u8>,
    pub range: Range,

    pub header: LuaHeader,
    pub functions: Vec<LuaFunction>
}

impl LuaBinary {
    pub fn update_targets(&mut self) {
        for function in self.functions.iter_mut() {
            function.update_targets();
        }
    }
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

        let header = LuaHeader {
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
        };

        stream.add_context(header.clone());
        Ok(header)
    }
}

impl ByteStreamRead for LuaLayout {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let instruction_size = header.instruction_size;
        // TODO: for future lua versions / different platforms
        // the instruction size may change
        
        if stream.is_out_of_bounds(instruction_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaLayout".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let raw = u32::read(stream)?;
        let opcode = LuaOpcode::from((raw & 0x3F) as u8);
        let a = ((raw >> 6) & 0xFF) as u8;

        match OPCODE_LAYOUT.get(&opcode) {
            Some(layout) => {
                match layout {
                    LuaLayout::A(_, _) => {
                        return Ok(LuaLayout::A(opcode, a));
                    },
                    LuaLayout::SBx(_, _) => {
                        let sbx = ((raw >> 14) & 0x3FFFF) as i32 - 131071;
                        return Ok(LuaLayout::SBx(opcode, sbx));
                    },
                    LuaLayout::AB(_, _, _) => {
                        let b = ((raw >> 23) & 0x1FF) as u16;
                        return Ok(LuaLayout::AB(opcode, a, b));
                    },
                    LuaLayout::AC(_, _, _) => {
                        let c = ((raw >> 14) & 0x1FF) as u16;
                        return Ok(LuaLayout::AC(opcode, a, c));
                    },
                    LuaLayout::ABx(_, _, _) => {
                        let bx = (raw >> 14) & 0x3FFFF;
                        return Ok(LuaLayout::ABx(opcode, a, bx as u32));
                    },
                    LuaLayout::AsBx(_, _, _) => {
                        let sbx = ((raw >> 14) & 0x3FFFF) as i32 - 131071;
                        return Ok(LuaLayout::AsBx(opcode, a, sbx));
                    },
                    LuaLayout::ABC(_, _, _, _) => {
                        let b = ((raw >> 23) & 0x1FF) as u16;
                        let c = ((raw >> 14) & 0x1FF) as u16;
                        return Ok(LuaLayout::ABC(opcode, a, b, c));
                    }
                }
            },
            None => {
                return Err(ByteStreamError::new(
                    stream, 
                    format!("unknown opcode: {}", opcode.to_string()).to_string(), 
                    ByteStreamErrorType::ReadFailure)
                );
            }
        }
    }
}

impl ByteStreamRead for LuaInstruction {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaInstruction".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let start = stream.caret();
        let raw = stream.peek(4).unwrap();
        let components = LuaLayout::read(stream)?;
        let opcode = match components {
            LuaLayout::A(opcode, _) => opcode,
            LuaLayout::SBx(opcode, _) => opcode,
            LuaLayout::AB(opcode, _, _) => opcode,
            LuaLayout::AC(opcode, _, _) => opcode,
            LuaLayout::ABx(opcode, _, _) => opcode,
            LuaLayout::AsBx(opcode, _, _) => opcode,
            LuaLayout::ABC(opcode, _, _, _) => opcode
        };

        let end = stream.caret();

        Ok(LuaInstruction {
            raw: raw,
            range: Range::new(start as u64, end as u64),

            opcode: opcode,
            components: components,
            pc: 0,

            jump_target: None
        })
    }
}

impl ByteStreamRead for LuaConstantType {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let number_size: u8 = header.lua_number_size;
        let size_t_size: u8 = header.size_t_size;

        let mut raw = Vec::new();
        let tag = u8::read(stream)?;
        raw.push(tag);

        match tag {
            0 => {
                return Ok(LuaConstantType::Nil(raw));
            },
            1 => {
                let value = u8::read(stream)?;
                raw.push(value);
                return Ok(LuaConstantType::Boolean(raw, value == 1));
            },
            3 => {
                if stream.is_out_of_bounds(number_size as usize) {
                    return Err(ByteStreamError::new(
                        stream, 
                        "not enough bytes to read LuaConstantType".to_string(), 
                        ByteStreamErrorType::OutOfBounds)
                    );
                }

                if number_size == 4 {
                    raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
                    let value = f32::read(stream)?;
                    return Ok(LuaConstantType::Number(raw, value as f64));
                } else if number_size == 8 {
                    raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
                    let value = f64::read(stream)?;
                    return Ok(LuaConstantType::Number(raw, value));
                } else {
                    return Err(ByteStreamError::new(
                        stream, 
                        format!("unsupported number size: {}", number_size).to_string(), 
                        ByteStreamErrorType::ReadFailure)
                    );
                }
            },
            4 => {
                // TODO: for future lua versions / different platforms
                if stream.is_out_of_bounds(size_t_size as usize) {
                    return Err(ByteStreamError::new(
                        stream, 
                        "not enough bytes to read LuaConstantType".to_string(), 
                        ByteStreamErrorType::OutOfBounds)
                    );
                }

                let mut bytes: Vec<u8> = Vec::new();
                let size = if size_t_size == 4 {
                    raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
                    u32::read(stream)? as u64
                } else if size_t_size == 8 {
                    raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
                    u64::read(stream)?
                } else {
                    return Err(ByteStreamError::new(
                        stream, 
                        format!("unsupported size_t size: {}", size_t_size).to_string(), 
                        ByteStreamErrorType::ReadFailure)
                    );
                };

                if stream.is_out_of_bounds(size as usize) {
                    return Err(ByteStreamError::new(
                        stream, 
                        "not enough bytes to read LuaConstantType".to_string(), 
                        ByteStreamErrorType::OutOfBounds)
                    );
                }

                for _ in 0..size {
                    let byte = u8::read(stream)?;
                    bytes.push(byte);
                }

                raw.extend_from_slice(bytes.as_slice());
                return Ok(LuaConstantType::String(raw, String::from_utf8(bytes).unwrap()));
            },
            _ => {
                return Err(ByteStreamError::new(
                    stream, 
                    format!("unknown constant tag: {}", tag).to_string(), 
                    ByteStreamErrorType::ReadFailure)
                );
            }
        }
    }
}

impl ByteStreamRead for LuaConstant {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let start = stream.caret();
        let constant = LuaConstantType::read(stream)?;
        let raw = match constant.clone() {
            LuaConstantType::Nil(raw) => raw,
            LuaConstantType::Boolean(raw, _) => raw,
            LuaConstantType::Number(raw, _) => raw,
            LuaConstantType::String(raw, _) => raw
        };
        let end = stream.caret();

        Ok(LuaConstant {
            raw: raw,
            range: Range::new(start as u64, end as u64),
            constant: constant
        })
    }
}

impl ByteStreamRead for LuaLocal {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;

        let start = stream.caret();
        let mut raw = Vec::new();

        let name_size = if size_t_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if size_t_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported size_t size: {}", size_t_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        if stream.is_out_of_bounds(name_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaLocal".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let name_bytes = stream.peek(name_size as usize).unwrap().to_vec();
        raw.extend_from_slice(name_bytes.as_slice());
        let name = String::from_utf8(name_bytes).unwrap();
        stream.skip(name_size as usize)?;

        let start_and_end_bytes = stream.peek(8).unwrap().to_vec();
        raw.extend_from_slice(start_and_end_bytes.as_slice());

        let start_pc = u32::read(stream)?;
        let end_pc = u32::read(stream)?;
        let end = stream.caret();

        Ok(LuaLocal {
            raw: raw,
            range: Range::new(start as u64, end as u64),

            name: name,
            start_pc: start_pc,
            end_pc: end_pc
        })
    }
}

impl ByteStreamRead for LuaUpvalue {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;

        let start = stream.caret();
        let mut raw = Vec::new();

        let name_size = if size_t_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if size_t_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported size_t size: {}", size_t_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        if stream.is_out_of_bounds(name_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaUpvalue".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let name_bytes = stream.peek(name_size as usize).unwrap().to_vec();
        raw.extend_from_slice(name_bytes.as_slice());
        let name = String::from_utf8(name_bytes).unwrap();
        stream.skip(name_size as usize)?;

        let end = stream.caret();

        Ok(LuaUpvalue {
            raw: raw,
            range: Range::new(start as u64, end as u64),

            name: name
        })
    }
}

impl ByteStreamRead for LuaFunction {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;
        let int_size: u8 = header.int_size;

        let start = stream.caret();
        let mut raw = Vec::new();

        if stream.is_out_of_bounds(size_t_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaFunction".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let name_size = if size_t_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if size_t_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported size_t size: {}", size_t_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        if stream.is_out_of_bounds(name_size as usize) {
            return Err(ByteStreamError::new(
                stream, 
                "not enough bytes to read LuaFunction".to_string(), 
                ByteStreamErrorType::OutOfBounds)
            );
        }

        let name_bytes = stream.peek(name_size as usize).unwrap().to_vec();
        raw.extend_from_slice(name_bytes.as_slice());
        let name = String::from_utf8(name_bytes).unwrap();
        stream.skip(name_size as usize)?;

        let first_line = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let last_line = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
        let num_upvalues = u8::read(stream)?;
        let num_parameters = u8::read(stream)?;
        let is_vararg = u8::read(stream)?;
        let max_stack_size = u8::read(stream)?;

        let code_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut code = Vec::new();
        for i in 0..code_size {
            let mut instruction = LuaInstruction::read(stream)?;
            instruction.pc = i;
            code.push(instruction);
        }

        let constant_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut constants = Vec::new();
        for _ in 0..constant_size {
            let constant = LuaConstant::read(stream)?;
            constants.push(constant);
        }

        let function_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut functions = Vec::new();
        for _ in 0..function_size {
            let function = LuaFunction::read(stream)?;
            functions.push(function);
        }

        let line_info_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut line_info = Vec::new();
        for _ in 0..line_info_size {
            let line = u32::read(stream)?;
            line_info.push(line);
        }

        let local_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut locals = Vec::new();
        for _ in 0..local_size {
            let local = LuaLocal::read(stream)?;
            locals.push(local);
        }

        let upvalue_size = if int_size == 4 {
            raw.extend_from_slice(stream.peek(4).unwrap().as_slice());
            u32::read(stream)? as u64
        } else if int_size == 8 {
            raw.extend_from_slice(stream.peek(8).unwrap().as_slice());
            u64::read(stream)?
        } else {
            return Err(ByteStreamError::new(
                stream, 
                format!("unsupported int size: {}", int_size).to_string(), 
                ByteStreamErrorType::ReadFailure)
            );
        };

        let mut upvalues = Vec::new();
        for _ in 0..upvalue_size {
            let upvalue = LuaUpvalue::read(stream)?;
            upvalues.push(upvalue);
        }

        let end = stream.caret();

        Ok(LuaFunction {
            raw: raw,
            range: Range::new(start as u64, end as u64),

            name: name,
            first_line: first_line,
            last_line: last_line,

            num_upvalues: num_upvalues,
            num_parameters: num_parameters,
            is_vararg: is_vararg,
            max_stack_size: max_stack_size,

            code_size: code_size,
            code: code,

            constant_size: constant_size,
            constants: constants,

            function_size: function_size,
            functions: functions,

            line_info_size: line_info_size,
            line_info: line_info,

            local_size: local_size,
            locals: locals,

            upvalue_size: upvalue_size,
            upvalues: upvalues
        })
    }
}

impl ByteStreamRead for LuaBinary {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let header = LuaHeader::read(stream)?;
        let entry = LuaFunction::read(stream)?;

        let start = header.range.start;
        let end = entry.range.end;

        fn add_functions(function: LuaFunction, functions: &mut Vec<LuaFunction>) {
            functions.push(function.clone());
            for f in function.functions {
                add_functions(f, functions);
            }
        }

        let mut functions = Vec::new();
        add_functions(entry, &mut functions);

        Ok(LuaBinary {
            raw: vec![],
            range: Range::new(start, end),

            header: header,
            functions: functions
        })
    }
}

impl ByteStreamWrite for LuaBinary {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        stream.clear_context();
        stream.add_context(self.header.clone());

        LuaHeader::write(&self.header, stream)?;
        let root = &self.functions[0];
        LuaFunction::write(root, stream)?;
        Ok(())
    }
}

impl ByteStreamWrite for LuaHeader {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let signature_bytes = self.signature.to_le_bytes();
        stream.write_bytes_slice(&signature_bytes)?;

        stream.write_byte(self.version)?;
        stream.write_byte(self.format)?;
        stream.write_byte(self.endianness)?;
        stream.write_byte(self.int_size)?;
        stream.write_byte(self.size_t_size)?;
        stream.write_byte(self.instruction_size)?;
        stream.write_byte(self.lua_number_size)?;
        stream.write_byte(self.integral_flag)?;

        Ok(())
    }
}

impl ByteStreamWrite for LuaLayout {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        match self {
            LuaLayout::A(opcode, a) => {
                let raw = (((*a as u32) << 6) | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::SBx(opcode, sbx) => {
                let raw = (((*sbx + 131071) as u32) << 14 | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::AB(opcode, a, b) => {
                let raw = (((*b as u32) << 23) | ((*a as u32) << 6) | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::AC(opcode, a, c) => {
                let raw = (((*c as u32) << 14) | ((*a as u32) << 6) | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::ABx(opcode, a, bx) => {
                let raw = ((*bx as u32) << 14 | (*a as u32) << 6 | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::AsBx(opcode, a, sbx) => {
                let raw = (((*sbx + 131071) as u32) << 14 | (*a as u32) << 6 | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            },
            LuaLayout::ABC(opcode, a, b, c) => {
                let raw = (((*c as u32) << 14) | ((*b as u32) << 23) | ((*a as u32) << 6) | (*opcode as u32)).to_le_bytes();
                stream.write_bytes_slice(&raw)?;
            }
        }
        Ok(())
    }
}

impl ByteStreamWrite for LuaInstruction {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        LuaLayout::write(&self.components, stream)?;
        Ok(())
    }
}

impl ByteStreamWrite for LuaConstantType {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;
        let number_size: u8 = header.lua_number_size;

        match self {
            LuaConstantType::Nil(_) => {
                stream.write_byte(0)?;
            },
            LuaConstantType::Boolean(_, value) => {
                stream.write_byte(1)?;
                stream.write_byte(if *value { 1 } else { 0 })?;
            },
            LuaConstantType::Number(_, value) => {
                stream.write_byte(3)?;
                let bytes = if number_size == 4 {
                    (*value as f32).to_le_bytes().to_vec()
                } else {
                    value.to_le_bytes().to_vec()
                };
                stream.write_bytes(bytes)?;
            },
            LuaConstantType::String(_, value) => {
                stream.write_byte(4)?;
                let bytes = if size_t_size == 4 {
                    (value.len() as u32).to_le_bytes().to_vec()
                } else {
                    (value.len() as u64).to_le_bytes().to_vec()
                };
                stream.write_bytes(bytes)?;
                stream.write_bytes_slice(value.as_bytes())?;
            }
        }
        Ok(())
    }
}

impl ByteStreamWrite for LuaConstant {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        LuaConstantType::write(&self.constant, stream)?;
        Ok(())
    }
}

impl ByteStreamWrite for LuaLocal {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;

        let bytes = if size_t_size == 4 {
            (self.name.len() as u32).to_le_bytes().to_vec()
        } else {
            (self.name.len() as u64).to_le_bytes().to_vec()
        };
        stream.write_bytes(bytes)?;

        stream.write_bytes_slice(self.name.as_bytes())?;
        stream.write_bytes_slice(&(self.start_pc as u32).to_le_bytes())?;
        stream.write_bytes_slice(&(self.end_pc as u32).to_le_bytes())?;
        Ok(())
    }
}

impl ByteStreamWrite for LuaUpvalue {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;

        let bytes = if size_t_size == 4 {
            (self.name.len() as u32).to_le_bytes().to_vec()
        } else {
            (self.name.len() as u64).to_le_bytes().to_vec()
        };
        stream.write_bytes(bytes)?;
        stream.write_bytes_slice(self.name.as_bytes())?;
        Ok(())
    }
}

impl ByteStreamWrite for LuaFunction {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let context = stream.get_context();
        let header: &LuaHeader = context[0];
        let size_t_size: u8 = header.size_t_size;
        let int_size: u8 = header.int_size;

        let bytes = if size_t_size == 4 {
            (self.name.len() as u32).to_le_bytes().to_vec()
        } else {
            (self.name.len() as u64).to_le_bytes().to_vec()
        };
        stream.write_bytes(bytes)?;
        stream.write_bytes_slice(self.name.as_bytes())?;

        if int_size == 4 {
            stream.write_bytes_slice(&(self.first_line as u32).to_le_bytes())?;
            stream.write_bytes_slice(&(self.last_line as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.first_line as u64).to_le_bytes())?;
            stream.write_bytes_slice(&(self.last_line as u64).to_le_bytes())?;
        }

        stream.write_byte(self.num_upvalues)?;
        stream.write_byte(self.num_parameters)?;
        stream.write_byte(self.is_vararg)?;
        stream.write_byte(self.max_stack_size)?;

        if int_size == 4 {
            stream.write_bytes_slice(&(self.code_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.code_size as u64).to_le_bytes())?;
        }

        for instruction in &self.code {
            LuaInstruction::write(instruction, stream)?;
        }

        if int_size == 4 {
            stream.write_bytes_slice(&(self.constant_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.constant_size as u64).to_le_bytes())?;
        }

        for constant in &self.constants {
            LuaConstant::write(constant, stream)?;
        }

        if int_size == 4 {
            stream.write_bytes_slice(&(self.function_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.function_size as u64).to_le_bytes())?;
        }

        for function in &self.functions {
            LuaFunction::write(function, stream)?;
        }

        if int_size == 4 {
            stream.write_bytes_slice(&(self.line_info_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.line_info_size as u64).to_le_bytes())?;
        }

        for line in &self.line_info {
            stream.write_bytes_slice(&line.to_le_bytes())?;
        }

        if int_size == 4 {
            stream.write_bytes_slice(&(self.local_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.local_size as u64).to_le_bytes())?;
        }

        for local in &self.locals {
            LuaLocal::write(local, stream)?;
        }

        if int_size == 4 {
            stream.write_bytes_slice(&(self.upvalue_size as u32).to_le_bytes())?;
        } else if int_size == 8 {
            stream.write_bytes_slice(&(self.upvalue_size as u64).to_le_bytes())?;
        }

        for upvalue in &self.upvalues {
            LuaUpvalue::write(upvalue, stream)?;
        }

        Ok(())
    }
}