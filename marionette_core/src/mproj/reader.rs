use bincode::de::read::Reader;
use bincode::{Decode};
use bincode::error::{DecodeError};
use crate::mproj::RawProject;

impl Decode for RawProject {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> core::result::Result<Self, bincode::error::DecodeError> {
        let project_version = bincode::Decode::decode(decoder)?;

        let mut bytes: Vec<u8> = Vec::new();
        let mut result: Result<u8, DecodeError> = Ok(0);
        while result.is_ok() {
            result = bincode::Decode::decode(decoder);
            match result {
                Ok(byte) => bytes.push(byte),
                Err(_) => break,
            }
        }

        Ok(RawProject {
            project_version,
            raw_project: bytes,
        })
    }
}