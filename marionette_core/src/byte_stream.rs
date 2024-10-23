// Purpose: serve as a byte stream utility for marionette
// src\byte_stream

use std::collections::VecDeque;

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
    pub endianness: Endian,

    /// The public context of the byte stream.
    /// This is used to store values that are necessary for functions using the byte stream.
    pub context: VecDeque<Box<dyn std::any::Any>>,
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
            endianness: stream.endianness.clone(),
            context: VecDeque::new() // context cannot be cloned
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
            endianness: Endian::Little,
            context: VecDeque::new()
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

    /// Returns the remaining bytes of the byte stream.
    /// 
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// u8::read(&mut byte_stream).unwrap();
    /// assert_eq!(byte_stream.remaining(), vec![0x01, 0x02, 0x03]);
    /// ```
    pub fn remaining(&self) -> Vec<u8> {
        self.bytes[self.index..].to_vec()
    }

    /// Rolls back the byte stream to a previous checkpoint.
    /// 
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// let b1 = u8::read(&mut byte_stream).unwrap();
    /// byte_stream.rollback(1);
    /// let b2 = u8::read(&mut byte_stream).unwrap();
    /// assert_eq!(b1, b2);
    /// ```
    pub fn rollback(&mut self, rollback: usize) {
        let mut rollback = rollback;
        while rollback > 0 && !self.history.is_empty() {
            let (index, size) = self.history.pop().unwrap();
            self.index = index;
            rollback -= 1;
        }
    }

    /// Adds an item to the context.
    pub fn add_context<T: 'static + std::any::Any>(&mut self, item: T) {
        self.context.push_back(Box::new(item));
    }

    /// Retrieves the context.
    pub fn get_context<T: 'static + std::any::Any>(&mut self) -> Vec<&T> {
        let mut context = Vec::new();
        for item in &self.context {
            if let Some(item) = item.downcast_ref::<T>() {
                context.push(item);
            }
        }

        context
    }
}

impl ToString for ByteStream {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for byte in &self.bytes {
            string.push(*byte as char);
        }
        string
    }
}