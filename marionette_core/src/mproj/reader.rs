use crate::byte_stream::{ByteStream, ByteStreamError, ByteStreamErrorType, ByteStreamRead, ByteStreamWrite};
use crate::mproj::RawProject;

use super::MarionetteProject;

impl ByteStreamRead for RawProject {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let raw = stream.bytes.clone();
        let project_version = String::read(stream)?;
        let remaining = stream.remaining();
        Ok(RawProject {
            project_version,
            raw,
            remaining,
        })
    }
}

impl ByteStreamRead for MarionetteProject {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let project_version = String::read(stream)?;
        
        let project_name = String::read(stream);
        if project_name.is_err() {
            return Err(ByteStreamError::new(stream, "Failed to read project name".to_string(), ByteStreamErrorType::ReadFailure));
        }
        let project_name = project_name.unwrap();
        
        let project_files = Vec::<String>::read(stream);
        if project_files.is_err() {
            return Err(ByteStreamError::new(stream, "Failed to read project files".to_string(), ByteStreamErrorType::ReadFailure));
        }
        let project_files = project_files.unwrap();

        Ok(MarionetteProject {
            project_version,
            project_name,
            project_files,
        })
    }
}