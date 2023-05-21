// Purpose: export types for the marionette disassembler library
// Path: src\exported_types.rs

use std::fmt;
use crate::byte_stream::ByteStream;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum DisassemblerErrorType {
    CannotDisassemble,

    ProblemReading,
    ProblemConverting,
    ProblemDisassembling,
    ByteStreamError,

    AnalysisError,

    BoxerError,

    NotImplemented,
    NotSupported
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DisassemblerError {
    pub address: u64,
    pub description: String,
    pub error_type: DisassemblerErrorType
}

#[repr(C)]
pub enum BValue {
    Null,
    UnsignedByte(u8),
    SignedByte(i8),
    UnsignedShort(u16),
    SignedShort(i16),
    UnsignedInt(u32),
    SignedInt(i32),
    UnsignedLong(u64),
    SignedLong(i64),
    Float(f32),
    Double(f64),
    String(String),
    Boolean(bool)
}

impl fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}: {}", self.address, self.description)
    }
}

impl std::error::Error for DisassemblerError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl From<i16> for DisassemblerError {
    fn from(value: i16) -> Self {
        DisassemblerError {
            address: value as u64,
            description: format!("boxer failed to convert {:0}", value),
            error_type: DisassemblerErrorType::BoxerError
        }
    }
}

impl DisassemblerError {
    pub fn not_implemented(address: u64, name: String) -> DisassemblerError {
        DisassemblerError {
            address,
            description: format!("{} is not implemented", name),
            error_type: DisassemblerErrorType::NotImplemented
        }
    }

    pub fn not_supported(address: u64, name: String) -> DisassemblerError {
        DisassemblerError {
            address,
            description: format!("{} is not supported", name),
            error_type: DisassemblerErrorType::NotSupported
        }
    }

    pub fn new(address: u64, description: String, error_type: DisassemblerErrorType) -> DisassemblerError {
        DisassemblerError {
            address,
            description,
            error_type
        }
    }
}