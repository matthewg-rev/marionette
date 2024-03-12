// Purpose: serve as a byte stream utility for marionette
// src\byte_stream

pub mod natives;
pub mod templated;

#[derive(Debug, Clone, PartialEq)]
pub enum Endian {
    Little,
    Big
}

#[derive(Debug, Clone, PartialEq)]
pub enum ByteStreamErrorType {
    HistoryFallbackFailure,
    OutOfBounds,
    ReadFailure,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ByteStreamError {
    pub address: usize,
    pub description: String,
    pub error_type: ByteStreamErrorType
}

impl ByteStreamError {
    pub fn new(stream: &mut ByteStream, description: String, error_type: ByteStreamErrorType) -> ByteStreamError {
        let address = stream.caret();
        ByteStreamError {
            address,
            description,
            error_type
        }
    }
}

impl std::fmt::Display for ByteStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}: {}", self.address, self.description)
    }
}

#[derive(Debug)]
/// A byte stream that reads from a vector of bytes.
pub struct ByteStream {
    /// A vector of bytes that the byte stream reads from.
    pub bytes: Vec<u8>,

    /// A vector of tuples that contains the index and the size of the previous reads.
    pub history: Vec<(usize, usize)>,

    /// The current index of the byte stream.
    pub index: usize,

    /// The endianness of the byte stream.
    /// Default is little endian.
    pub endianness: Endian
}

pub trait ByteStreamRead: Sized {
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError>;
}

pub trait ByteStreamWrite {
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError>;
}

impl From<&ByteStream> for ByteStream {
    fn from(stream: &ByteStream) -> ByteStream {
        ByteStream {
            bytes: stream.bytes.clone(),
            history: stream.history.clone(),
            index: 0,
            endianness: stream.endianness.clone()
        }
    }
}

impl ByteStream {
    /// Creates a new byte stream from a vector of bytes.
    pub fn new(bytes: Vec<u8>) -> ByteStream {
        ByteStream {
            bytes,
            history: Vec::new(),
            index: 0,
            endianness: Endian::Little
        }
    }

    /// Returns whether the current index with an offset is out of bounds.
    ///
    /// # Arguments
    /// * `size` - The size of the offset.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.is_out_of_bounds(0), false);
    /// assert_eq!(byte_stream.is_out_of_bounds(5), true);
    /// ```
    pub fn is_out_of_bounds(&self, size: usize) -> bool {
        self.index + size > self.bytes.len()
    }

    /// Returns the current index of the byte stream.
    /// 
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.caret(), 0);
    /// ```
    pub fn caret(&self) -> usize {
        self.index
    }

    pub fn remaining(&self) -> Vec<u8> {
        self.bytes[self.index..].to_vec()
    }
}