// Purpose: serve as a byte stream utility for marionette
// src\byte_stream

use bincode::{Decode, Encode};
use bincode::error::DecodeError;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteStreamErrorType {
    HistoryFallbackFailure,
    OutOfBounds,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ByteStreamError {
    pub address: u64,
    pub description: String,
    pub error_type: ByteStreamErrorType
}

impl ByteStreamError {
    pub fn new(stream: &mut ByteStream, description: String, error_type: ByteStreamErrorType) -> ByteStreamError {
        let address = stream.current_address();
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
    bytes: Vec<u8>,

    /// A vector of tuples that contains the index and the size of the previous reads.
    history: Vec<(usize, usize)>,

    /// The current index of the byte stream.
    index: usize
}

impl ByteStream {
    /// Creates a new byte stream from a vector of bytes.
    pub fn new(bytes: Vec<u8>) -> ByteStream {
        ByteStream {
            bytes,
            history: Vec::new(),
            index: 0
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

    /// Consumes a byte from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u8().unwrap(), 0x00);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// while let Ok(_) = byte_stream.read_u8() {}
    /// assert_eq!(byte_stream.read_u8().unwrap_err().error_type, ByteStreamErrorType::OutOfBounds);
    /// ```
    pub fn read_u8(&mut self) -> Result<u8, ByteStreamError> {
        if self.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = self.bytes[self.index];
        self.index += 1;
        self.history.push((self.index, 1));
        Ok(result)
    }

    /// Consumes a unsigned 16-bit integer from the byte stream and returns it. (Little Endian BA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u16().unwrap(), 256);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u16().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_u16(&mut self) -> Result<u16, ByteStreamError> {
        if self.is_out_of_bounds(2) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = u16::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
        self.index += 2;
        self.history.push((self.index, 2));
        Ok(result)
    }

    /// Consumes a unsigned 32-bit integer from the byte stream and returns it. (Little Endian DCBA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u32().unwrap(), 50462976);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u32().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_u32(&mut self) -> Result<u32, ByteStreamError> {
        if self.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = u32::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3]]);
        self.index += 4;
        self.history.push((self.index, 4));
        Ok(result)
    }

    /// Consumes a unsigned 64-bit integer from the byte stream and returns it. (Little Endian HGFEDCBA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u64().unwrap(), 506097522914230528);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u64().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_u64(&mut self) -> Result<u64, ByteStreamError> {
        if self.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = u64::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3], self.bytes[self.index + 4], self.bytes[self.index + 5], self.bytes[self.index + 6], self.bytes[self.index + 7]]);
        self.index += 8;
        self.history.push((self.index, 8));
        Ok(result)
    }

    /// Reads a 32-bit floating point number from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x00, 0x80, 0x3F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_f32().unwrap(), 1.0);
    /// ```
    pub fn read_f32(&mut self) -> Result<f32, ByteStreamError> {
        if self.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = f32::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3]]);
        self.index += 4;
        self.history.push((self.index, 4));
        Ok(result)
    }

    /// Reads a 64-bit floating point number from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00,0xF0, 0x3F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_f64().unwrap(), 1.0);
    /// ```
    pub fn read_f64(&mut self) -> Result<f64, ByteStreamError> {
        if self.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = f64::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3], self.bytes[self.index + 4], self.bytes[self.index + 5], self.bytes[self.index + 6], self.bytes[self.index + 7]]);
        self.index += 8;
        self.history.push((self.index, 8));
        Ok(result)
    }

    /// Consumes a signed 8-bit integer from the byte stream and returns it. (Little Endian)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i8().unwrap(), 0);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i8().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_i8(&mut self) -> Result<i8, ByteStreamError> {
        if self.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = self.bytes[self.index] as i8;
        self.index += 1;
        self.history.push((self.index, 1));
        Ok(result)
    }

    /// Consumes a signed 16-bit integer from the byte stream and returns it. (Little Endian BA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i16().unwrap(), 256);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i16().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_i16(&mut self) -> Result<i16, ByteStreamError> {
        if self.is_out_of_bounds(2) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = i16::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
        self.index += 2;
        self.history.push((self.index, 2));
        Ok(result)
    }

    /// Consumes a signed 32-bit integer from the byte stream and returns it. (Little Endian BADC)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i32().unwrap(), 66051);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i32().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_i32(&mut self) -> Result<i32, ByteStreamError> {
        if self.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = i32::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3]]);
        self.index += 4;
        self.history.push((self.index, 4));
        Ok(result)
    }

    /// Consumes a signed 64-bit integer from the byte stream and returns it. (Little Endian HGFEDCBA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x6, 0x07];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i64().unwrap(), 506097522914230528);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i64().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_i64(&mut self) -> Result<i64, ByteStreamError> {
        if self.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let result = i64::from_le_bytes(self.read_bytes(8)?.try_into().unwrap());
        self.index += 8;
        self.history.push((self.index, 8));
        Ok(result)
    }

    /// Consumes a specified number of bytes from the byte stream and returns them.
    /// # Arguments
    /// * `count` - The number of bytes to consume.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_bytes(2).unwrap(), vec![0x00, 0x01]);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_bytes(4).unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, ByteStreamError> {
        if self.is_out_of_bounds(count) {
            return Err(ByteStreamError::new(self, "index out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        self.index += count;
        self.history.push((self.index, count));
        Ok(self.bytes[self.index - count..self.index].to_vec())
    }

    /// Allows us to essentially go back in time to a previous state of the byte stream.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// // We can peek at the next 32-bit integer without consuming it.
    /// let bytes = vec![0x00, 0x40, 0x00, 0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// let peeked = byte_stream.read_i32().unwrap();
    /// assert_eq!(peeked, 16384);
    /// assert_eq!(byte_stream.revert().unwrap(), ());
    ///
    /// // We have now successfully read and reverted back to the original state.
    /// println!("{}", byte_stream.read_i32().unwrap()); // 16384
    /// ```
    pub fn revert(&mut self) -> Result<(), ByteStreamError> {
        if self.history.is_empty() {
            return Err(ByteStreamError::new(self, "no history to revert".to_string(), ByteStreamErrorType::HistoryFallbackFailure));
        }

        let (index, count) = self.history.pop().unwrap();
        self.index = index - count;
        Ok(())
    }

    /// Consumes a unsigned little endian base 128 integer from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x80, 0x80, 0x80, 0x80, 0x08];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_uleb128().unwrap(), 2147483648);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_uleb128().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_uleb128(&mut self) -> Result<u64, ByteStreamError> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(result)
    }

    /// Consumes a signed little endian base 128 integer from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x80, 0x80, 0x80, 0x80, 0x08];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_sleb128().unwrap(), 2147483648);
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_sleb128().unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_sleb128(&mut self) -> Result<i64, ByteStreamError> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                if shift < 64 && (byte & 0x40) != 0 {
                    result |= -1_i64 << shift;
                }
                break;
            }
        }
        Ok(result)
    }

    /// Consumes a specified amount of bytes from the byte stream and returns them as a string.
    /// # Arguments
    /// * `count` - The length of the string to read.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_as_string(5).unwrap(), "Hello".to_string());
    /// ```
    /// # Errors
    /// Returns a `ByteStreamError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamErrorType};
    ///
    /// let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_as_string(6).unwrap_err().error_type, ByteStreamErrorType::ByteStreamError);
    /// ```
    pub fn read_as_string(&mut self, count: usize) -> Result<String, ByteStreamError> {
        let bytes = self.read_bytes(count)?;
        let mut result = String::new();
        for byte in bytes {
            result.push(byte as char);
        }
        Ok(result)
    }

    /// Takes a unsigned 32 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::u32_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00, 0x00, 0x00]);
    /// ```
    pub fn u32_to_bytes(value: u32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a unsigned 64 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::u64_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// ```
    pub fn u64_to_bytes(value: u64) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a signed 32 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::i32_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00, 0x00, 0x00]);
    /// ```
    pub fn i32_to_bytes(value: i32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a signed 64 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::i64_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// ```
    pub fn i64_to_bytes(value: i64) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a 32 bit floating point number and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::f32_to_bytes(1.0);
    /// assert_eq!(bytes, vec![0x00, 0x00, 0x80, 0x3F]);
    /// ```
    pub fn f32_to_bytes(value: f32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a 64 bit floating point number and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::f64_to_bytes(1.0);
    /// assert_eq!(bytes, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]);
    /// ```
    pub fn f64_to_bytes(value: f64) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a unsigned 16 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::u16_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00]);
    /// ```
    pub fn u16_to_bytes(value: u16) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a signed 16 bit integer and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::i16_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01, 0x00]);
    /// ```
    pub fn i16_to_bytes(value: i16) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Takes a byte and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::u8_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01]);
    /// ```
    pub fn u8_to_bytes(value: u8) -> Vec<u8> {
        vec![value]
    }

    /// Takes a signed byte and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::i8_to_bytes(1);
    /// assert_eq!(bytes, vec![0x01]);
    /// ```
    pub fn i8_to_bytes(value: i8) -> Vec<u8> {
        vec![value as u8]
    }

    /// Takes a string and returns it as a vector of bytes.
    /// # Arguments
    /// * `value` - The value to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = ByteStream::string_to_bytes(String::from("Hello World!"));
    /// assert_eq!(bytes, vec![
    ///     0x48, 0x65, 0x6C, 0x6C,
    ///     0x6F, 0x20, 0x57, 0x6F,
    ///     0x72, 0x6C, 0x64, 0x21
    /// ]);
    /// ```
    pub fn string_to_bytes(value: String) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    /// Takes a vector of bytes and returns it as a unsigned 32 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00];
    /// let value = ByteStream::bytes_to_u32(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a unsigned 64 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// let value = ByteStream::bytes_to_u64(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a signed 32 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00];
    /// let value = ByteStream::bytes_to_i32(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a signed 64 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// let value = ByteStream::bytes_to_i64(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_i64(bytes: &[u8]) -> i64 {
        i64::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a 32 bit floating point number.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x00, 0x80, 0x3F];
    /// let value = ByteStream::bytes_to_f32(&bytes);
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn bytes_to_f32(bytes: &[u8]) -> f32 {
        f32::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a 64 bit floating point number.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, 0x40];
    /// let value = ByteStream::bytes_to_f64(&bytes);
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn bytes_to_f64(bytes: &[u8]) -> f64 {
        f64::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a unsigned 16 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00];
    /// let value = ByteStream::bytes_to_u16(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a signed 16 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00];
    /// let value = ByteStream::bytes_to_i16(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
        i16::from_le_bytes(bytes.try_into().unwrap())
    }

    /// Takes a vector of bytes and returns it as a unsigned 8 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01];
    /// let value = ByteStream::bytes_to_u8(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_u8(bytes: &[u8]) -> u8 {
        bytes[0]
    }

    /// Takes a vector of bytes and returns it as a signed 8 bit integer.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01];
    /// let value = ByteStream::bytes_to_i8(&bytes);
    /// assert_eq!(value, 1);
    /// ```
    pub fn bytes_to_i8(bytes: &[u8]) -> i8 {
        bytes[0] as i8
    }

    /// Takes a vector of bytes and returns it as a string.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![
    ///     0x48, 0x65, 0x6C, 0x6C,
    ///     0x6F, 0x20, 0x57, 0x6F,
    ///     0x72, 0x6C, 0x64, 0x21
    /// ];
    /// let value = ByteStream::bytes_to_string(&bytes);
    /// assert_eq!(value, "Hello World!");
    /// ```
    pub fn bytes_to_string(bytes: &[u8]) -> String {
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    /// Takes a vector of bytes and returns it as a vector of unsigned 16 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x02, 0x00];
    /// let value = ByteStream::u8_vec_to_u16_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_u16_vec(bytes: Vec<u8>) -> Vec<u16> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 2 {
            result.push(Self::bytes_to_u16(&bytes[i * 2..i * 2 + 2]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of unsigned 32 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    /// let value = ByteStream::u8_vec_to_u32_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_u32_vec(bytes: Vec<u8>) -> Vec<u32> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 4 {
            result.push(Self::bytes_to_u32(&bytes[i * 4..i * 4 + 4]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of unsigned 64 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![
    ///     0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    /// ];
    /// let value = ByteStream::u8_vec_to_u64_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_u64_vec(bytes: Vec<u8>) -> Vec<u64> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 8 {
            result.push(Self::bytes_to_u64(&bytes[i * 8..i * 8 + 8]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of signed 16 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x02, 0x00];
    /// let value = ByteStream::u8_vec_to_i16_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_i16_vec(bytes: Vec<u8>) -> Vec<i16> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 2 {
            result.push(Self::bytes_to_i16(&bytes[i * 2..i * 2 + 2]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of signed 32 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    /// let value = ByteStream::u8_vec_to_i32_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_i32_vec(bytes: Vec<u8>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 4 {
            result.push(Self::bytes_to_i32(&bytes[i * 4..i * 4 + 4]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of signed 64 bit integers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![
    ///    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    /// ];
    /// let value = ByteStream::u8_vec_to_i64_vec(bytes);
    /// assert_eq!(value, vec![1, 2]);
    /// ```
    pub fn u8_vec_to_i64_vec(bytes: Vec<u8>) -> Vec<i64> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 8 {
            result.push(Self::bytes_to_i64(&bytes[i * 8..i * 8 + 8]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of 32 bit floating point numbers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![
    ///    0x00, 0x00, 0x80, 0x3F,
    ///    0x00, 0x00, 0x00, 0x40
    /// ];
    /// let value = ByteStream::u8_vec_to_f32_vec(bytes);
    /// assert_eq!(value, vec![1.0, 2.0]);
    /// ```
    pub fn u8_vec_to_f32_vec(bytes: Vec<u8>) -> Vec<f32> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 4 {
            result.push(Self::bytes_to_f32(&bytes[i * 4..i * 4 + 4]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of 64 bit floating point numbers.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40
    /// ];
    /// let value = ByteStream::u8_vec_to_f64_vec(bytes);
    /// assert_eq!(value, vec![1.0, 2.0]);
    /// ```
    pub fn u8_vec_to_f64_vec(bytes: Vec<u8>) -> Vec<f64> {
        let mut result = Vec::new();
        for i in 0..bytes.len() / 8 {
            result.push(Self::bytes_to_f64(&bytes[i * 8..i * 8 + 8]));
        }
        result
    }

    /// Takes a vector of bytes and returns it as a vector of booleans.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00];
    /// let value = ByteStream::u8_vec_to_bool_vec(bytes);
    /// assert_eq!(value, vec![true, false]);
    /// ```
    pub fn u8_vec_to_bool_vec(bytes: Vec<u8>) -> Vec<bool> {
        let mut result = Vec::new();
        bytes.iter().for_each(|b| {
            result.push(*b != 0);
        });
        result
    }

    /// Takes a vector of bytes and return the entropy measurement of the block.
    /// # Arguments
    /// * `bytes` - A vector of bytes to convert.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let bytes = vec![0x01, 0x00];
    /// let value = ByteStream::get_entropy_of_block(bytes);
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn get_entropy_of_block(bytes: Vec<u8>) -> f32 {
        let mut entropy = 0.0;
        let mut counts = [0; 256];
        bytes.iter().for_each(|b| counts[*b as usize] += 1);
        counts.iter().for_each(|c| {
            if *c != 0 {
                entropy
                    += (*c as f32 / bytes.len() as f32)
                    * (*c as f32 / bytes.len() as f32).log2();
            }
        });
        -entropy
    }

    /// Returns the total entropy of the Byte Stream
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![
    ///     0x01, 0x02, 0x03, 0x04,
    ///     0x05, 0x06, 0x07, 0x08
    /// ]);
    /// assert_eq!(bs.get_total_entropy(), 3.0);
    /// ```
    pub fn get_total_entropy(&self) -> f32 {
        Self::get_entropy_of_block(self.bytes.clone())
    }

    /// Returns the entropy of a block of bytes
    /// # Arguments
    /// * `start_address` - The start address of the block
    /// * `end_address` - The end address of the block
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![
    ///     0x00, 0x00, 0x00, 0x00, // padding
    ///     0x01, 0x02, 0x03, 0x04,
    ///     0x05, 0x06, 0x07, 0x08,
    ///     0x00, 0x00, 0x00, 0x00  // padding
    /// ]);
    /// assert_eq!(bs.get_block_entropy(4, 12), 3.0);
    /// ```
    pub fn get_block_entropy(&self, start_address: u64, end_address: u64) -> f32 {
        let mut block = vec![];
        for i in start_address..end_address {
            block.push(self.bytes[i as usize]);
        }
        Self::get_entropy_of_block(block)
    }

    /// Returns the current index of the Byte Stream
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.read_u8();
    /// assert_eq!(bs.current_address(), 1);
    /// ```
    pub fn current_address(&self) -> u64 {
        self.index as u64
    }

    /// Returns a copy of the bytes
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Writes a u8 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_u8(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0]);
    /// ```
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Writes a u16 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_u16(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0]);
    /// ```
    pub fn write_u16(&mut self, value: u16) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a u32 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_u32(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0]);
    /// ```
    pub fn write_u32(&mut self, value: u32) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a u64 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_u64(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn write_u64(&mut self, value: u64) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a uleb128 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_uleb128(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0]);
    /// ```
    pub fn write_uleb128(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.bytes.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    /// Writes a i8 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_i8(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0]);
    /// ```
    pub fn write_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    /// Writes a i16 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_i16(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0]);
    /// ```
    pub fn write_i16(&mut self, value: i16) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a i32 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_i32(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0]);
    /// ```
    pub fn write_i32(&mut self, value: i32) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a i64 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_i64(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn write_i64(&mut self, value: i64) {
        value.to_le_bytes().iter().for_each(|b| self.bytes.push(*b));
    }

    /// Writes a sleb128 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_sleb128(0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0]);
    /// ```
    pub fn write_sleb128(&mut self, mut value: i64) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            let more = !(((value == 0) && ((byte & 0x40) == 0)) || ((value == -1) && ((byte & 0x40) != 0)));
            if more {
                byte |= 0x80;
            }
            self.bytes.push(byte);
            if !more {
                break;
            }
        }
    }

    /// Writes a f32 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_f32(0.0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0 ,0]);
    /// ```
    pub fn write_f32(&mut self, value: f32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a f64 to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_f64(0.0);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0 ,0, 0, 0, 0, 0]);
    /// ```
    pub fn write_f64(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a string to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_string("Hello World!");
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
    /// ```
    pub fn write_string(&mut self, value: &str) {
        self.bytes.extend_from_slice(value.as_bytes());
    }

    /// Writes a byte array to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.write_bytes(vec![8, 9, 10, 11]);
    /// assert_eq!(bs.get_bytes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ,11]);
    /// ```
    pub fn write_bytes(&mut self, value: Vec<u8>) {
        self.bytes.extend_from_slice(&value);
    }

    /// Writes a struct to the Byte Stream
    /// # Arguments
    /// * `value` - The value to write
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use bincode::{Encode};
    ///
    /// #[derive(Encode)]
    /// struct TestStruct {
    ///     name: String
    /// }
    ///
    /// let mut bs = ByteStream::new(vec![]);
    /// bs.write_struct(TestStruct { name: "Hello World!".to_string() });
    /// assert_eq!(bs.get_bytes(), vec![12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
    /// ```
    pub fn write_struct<T: Encode>(&mut self, value: T) {
        let config = bincode::config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&value, config).unwrap();
        self.write_bytes(encoded);
    }

    /// Reads a struct from the Byte Stream
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use bincode::{Encode, Decode};
    ///
    /// #[derive(Encode, Decode, PartialEq, Debug)]
    /// struct TestStruct {
    ///     name: String
    /// }
    ///
    /// let mut bs = ByteStream::new(vec![12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
    /// let decoded: TestStruct = bs.read_struct().unwrap();
    /// assert_eq!(decoded, TestStruct { name: "Hello World!".to_string() });
    /// assert_eq!(bs.current_address() == 13, true);
    /// ```
    pub fn read_struct<T: Decode>(&mut self) -> Result<T, DecodeError> {
        let config = bincode::config::standard();
        let decoded: Result<T, DecodeError> = bincode::decode_from_std_read(self, config);
        decoded
    }

    /// Jump to a specific address in the Byte Stream
    /// # Arguments
    /// * `address` - The address to jump to
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// let mut bs = ByteStream::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// bs.set_address(4);
    /// assert_eq!(bs.current_address(), 4);
    /// ```
    pub fn set_address(&mut self, address: u64) {
        self.index = address as usize;
    }

    pub fn to_vec_u16(&mut self) -> Vec<u16> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 2 {
            vec.push(self.read_u16().unwrap());
        }
        vec
    }

    pub fn to_vec_u32(&mut self) -> Vec<u32> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 4 {
            vec.push(self.read_u32().unwrap());
        }
        vec
    }

    pub fn to_vec_u64(&mut self) -> Vec<u64> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 8 {
            vec.push(self.read_u64().unwrap());
        }
        vec
    }

    pub fn to_vec_i16(&mut self) -> Vec<i16> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 2 {
            vec.push(self.read_i16().unwrap());
        }
        vec
    }

    pub fn to_vec_i32(&mut self) -> Vec<i32> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 4 {
            vec.push(self.read_i32().unwrap());
        }
        vec
    }

    pub fn to_vec_i64(&mut self) -> Vec<i64> {
        let mut vec = vec![];
        for _ in 0..self.bytes.len() / 8 {
            vec.push(self.read_i64().unwrap());
        }
        vec
    }

    pub fn hex_dump(bytes: Vec<u8>) -> String {
        let mut output = String::new();
        bytes.chunks(16).enumerate().for_each(|(i, chunk)| {
            output.push_str(&format!("{:08X}  ", i * 16));
            chunk.iter().for_each(|byte| {
                output.push_str(&format!("{:02X} ", byte));
            });
            // add empty space if we didn't get a full chunk
            if chunk.len() < 16 {
                for _ in 0..(16 - chunk.len()) {
                    output.push_str("   ");
                }
            }

            output.push_str("  ");
            chunk.iter().for_each(|byte| {
                if *byte >= 0x20 && *byte <= 0x7E {
                    output.push(*byte as char);
                } else {
                    output.push('.');
                }
            });

            output.push('\n');
        });
        output
    }

    pub fn sync(&mut self, other: &mut ByteStream) {
        if self.bytes != other.bytes {
            return;
        }
        other.set_address(self.current_address());
    }
}

impl std::io::Read for ByteStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.read_bytes(buf.len()).unwrap();
        buf.copy_from_slice(&bytes);
        Ok(bytes.len())
    }
}