use crate::mproj::MarionetteProject;
use crate::byte_stream::{ByteStream, ByteStreamError, ByteStreamErrorType, ByteStreamRead, ByteStreamWrite};

impl ByteStreamWrite for MarionetteProject {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        self.project_version.write(stream)?;
        self.project_name.write(stream)?;
        self.project_files.write(stream)?;
        Ok(())
    }
}