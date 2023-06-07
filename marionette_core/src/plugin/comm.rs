use bincode::{Decode, Encode};

pub struct CompilerInfo {
    pub name: String,
    pub major: u8,
    pub minor: u8,
}

impl Encode for CompilerInfo {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.name, encoder)?;
        bincode::Encode::encode(&self.major, encoder)?;
        bincode::Encode::encode(&self.minor, encoder)?;
        Ok(())
    }
}

impl Decode for CompilerInfo {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> core::result::Result<Self, bincode::error::DecodeError> {
        let name = bincode::Decode::decode(decoder)?;
        let major = bincode::Decode::decode(decoder)?;
        let minor = bincode::Decode::decode(decoder)?;
        Ok(CompilerInfo {
            name,
            major,
            minor
        })
    }
}