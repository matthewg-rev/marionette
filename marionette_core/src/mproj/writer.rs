use bincode::{Encode};
use crate::mproj::MarionetteProject;

impl Encode for MarionetteProject {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        // for compatibility checking, the version of the project MUST be encoded first
        bincode::Encode::encode(&self.project_version, encoder)?;

        bincode::Encode::encode(&self.project_name, encoder)?;
        bincode::Encode::encode(&self.project_files, encoder)?;
        Ok(())
    }
}