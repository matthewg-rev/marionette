// Purpose: serve as a byte stream utility for marionette
// src\byte_stream

use crate::exported_types::{DisassemblerError, DisassemblerErrorType};

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
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// while let Ok(_) = byte_stream.read_u8() {}
    /// assert_eq!(byte_stream.read_u8().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_u8(&mut self) -> Result<u8, DisassemblerError> {
        if self.is_out_of_bounds(1) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
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
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u16().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_u16(&mut self) -> Result<u16, DisassemblerError> {
        if self.is_out_of_bounds(2) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
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
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u32().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_u32(&mut self) -> Result<u32, DisassemblerError> {
        if self.is_out_of_bounds(4) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), crate::exported_types::DisassemblerErrorType::ByteStreamError));
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
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_u64().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_u64(&mut self) -> Result<u64, DisassemblerError> {
        if self.is_out_of_bounds(8) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
        }

        let result = u64::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3], self.bytes[self.index + 4], self.bytes[self.index + 5], self.bytes[self.index + 6], self.bytes[self.index + 7]]);
        self.index += 8;
        self.history.push((self.index, 8));
        Ok(result)
    }

    /// Consumes a signed 8-bit integer from the byte stream and returns it. (Little Endian)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i8().unwrap(), 0);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i8().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_i8(&mut self) -> Result<i8, DisassemblerError> {
        if self.is_out_of_bounds(1) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
        }

        let result = self.bytes[self.index] as i8;
        self.index += 1;
        self.history.push((self.index, 1));
        Ok(result)
    }

    /// Consumes a signed 16-bit integer from the byte stream and returns it. (Little Endian BA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i16().unwrap(), 256);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i16().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_i16(&mut self) -> Result<i16, DisassemblerError> {
        if self.is_out_of_bounds(2) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
        }

        let result = i16::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
        self.index += 2;
        self.history.push((self.index, 2));
        Ok(result)
    }

    /// Consumes a signed 32-bit integer from the byte stream and returns it. (Little Endian BADC)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i32().unwrap(), 66051);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i32().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_i32(&mut self) -> Result<i32, DisassemblerError> {
        if self.is_out_of_bounds(4) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
        }

        let result = i32::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1], self.bytes[self.index + 2], self.bytes[self.index + 3]]);
        self.index += 4;
        self.history.push((self.index, 4));
        Ok(result)
    }

    /// Consumes a signed 64-bit integer from the byte stream and returns it. (Little Endian HGFEDCBA)
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x6, 0x07];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i64().unwrap(), 506097522914230528);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_i64().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_i64(&mut self) -> Result<i64, DisassemblerError> {
        if self.is_out_of_bounds(8) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
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
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02, 0x03];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_bytes(2).unwrap(), vec![0x00, 0x01]);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x00, 0x01, 0x02];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_bytes(4).unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, DisassemblerError> {
        if self.is_out_of_bounds(count) {
            return Err(DisassemblerError::new(self.index as u64, "index out of bounds".to_string(), DisassemblerErrorType::ByteStreamError));
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
    pub fn revert(&mut self) -> Result<(), DisassemblerError> {
        if self.history.is_empty() {
            return Err(DisassemblerError::new(self.index as u64, "no history to revert".to_string(), DisassemblerErrorType::ByteStreamError));
        }

        let (index, count) = self.history.pop().unwrap();
        self.index = index - count;
        Ok(())
    }

    /// Consumes a unsigned little endian base 128 integer from the byte stream and returns it.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x80, 0x80, 0x80, 0x80, 0x08];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_uleb128().unwrap(), 2147483648);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_uleb128().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_uleb128(&mut self) -> Result<u64, DisassemblerError> {
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
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x80, 0x80, 0x80, 0x80, 0x08];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_sleb128().unwrap(), 2147483648);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_sleb128().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_sleb128(&mut self) -> Result<i64, DisassemblerError> {
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
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_as_string(5).unwrap(), "Hello".to_string());
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_as_string(6).unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_as_string(&mut self, count: usize) -> Result<String, DisassemblerError> {
        let bytes = self.read_bytes(count)?;
        let mut result = String::new();
        for byte in bytes {
            result.push(byte as char);
        }
        Ok(result)
    }

    /// Consumes a computed amount of bytes from the byte stream and returns them as the specified struct.
    /// # Type Parameters
    /// * `T` - The type of the struct to read.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    ///
    /// #[derive(Debug)]
    /// struct Vector2 {
    ///    x: i16,
    ///    y: i16
    /// };
    ///
    /// let bytes = vec![0x01, 0x00, 0x02, 0x00];
    /// let mut byte_stream = ByteStream::new(bytes);
    ///
    /// let vector = byte_stream.read_struct::<Vector2>().unwrap();
    /// assert_eq!(vector.x, 1);
    /// assert_eq!(vector.y, 2);
    /// ```
    /// # Errors
    /// Returns a `DisassemblerError` if the index is out of bounds.
    /// ```
    /// use marionette_core::byte_stream::ByteStream;
    /// use marionette_core::exported_types::DisassemblerErrorType;
    ///
    /// #[derive(Debug)]
    /// struct Vector2 {
    ///   x: i16,
    ///   y: i32
    /// };
    ///
    /// let bytes = vec![];
    /// let mut byte_stream = ByteStream::new(bytes);
    /// assert_eq!(byte_stream.read_struct::<Vector2>().unwrap_err().error_type, DisassemblerErrorType::ByteStreamError);
    /// ```
    pub fn read_struct<T>(&mut self) -> Result<T, DisassemblerError> {
        let size = std::mem::size_of::<T>();
        let bytes = self.read_bytes(size)?;

        Ok(unsafe { std::ptr::read(bytes.as_ptr() as *const T) })
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
        /* we are getting the entropy of a set of bytes
         * hence we have options from 0->256
         * we are using the shannon entropy formula
         */
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
}