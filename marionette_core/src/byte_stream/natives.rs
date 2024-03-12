use crate::byte_stream::{ByteStream, ByteStreamError, ByteStreamErrorType, ByteStreamRead, ByteStreamWrite, Endian};

impl ByteStreamRead for u8 {
    /// Reader for u8 type 'byte' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03]);
    /// let mut byte = u8::read(&mut stream).unwrap();
    /// assert_eq!(byte, 0x00);
    /// byte = u8::read(&mut stream).unwrap();
    /// assert_eq!(byte, 0x01);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        let value = stream.bytes[stream.index];
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(value)
    }
}

impl ByteStreamWrite for u8 {
    /// Writer for u8 type 'byte' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let byte: u8 = 0xFF;
    /// byte.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        stream.bytes.push(*self);
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(())
    }
}

impl ByteStreamRead for u16 {
    /// Reader for u16 type 'word' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03]);
    /// let mut word = u16::read(&mut stream).unwrap();
    /// assert_eq!(word, 0x0100);
    /// word = u16::read(&mut stream).unwrap();
    /// assert_eq!(word, 0x0302);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(2) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            u16::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1]])
        } else {
            u16::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1]])
        };
        stream.history.push((stream.index, 2));
        stream.index += 2;
        Ok(value)
    }
}

impl ByteStreamWrite for u16 {
    /// Writer for u16 type 'word' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let word: u16 = 0xFF00;
    /// word.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 2));
        stream.index += 2;
        Ok(())
    }
}

impl ByteStreamRead for u32 {
    /// Reader for u32 type 'dword' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    /// let mut dword = u32::read(&mut stream).unwrap();
    /// assert_eq!(dword, 0x03020100);
    /// dword = u32::read(&mut stream).unwrap();
    /// assert_eq!(dword, 0x07060504);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            u32::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        } else {
            u32::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        };
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(value)
    }
}

impl ByteStreamWrite for u32 {
    /// Writer for u32 type 'dword' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let dword: u32 = 0xFF000000;
    /// dword.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0x00, 0x00, 0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(())
    }
}

impl ByteStreamRead for u64 {
    /// Reader for u64 type 'qword' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
    /// let mut qword = u64::read(&mut stream).unwrap();
    /// assert_eq!(qword, 0x0706050403020100);
    /// qword = u64::read(&mut stream).unwrap();
    /// assert_eq!(qword, 0x0F0E0D0C0B0A0908);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            u64::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        } else {
            u64::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        };
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(value)
    }
}

impl ByteStreamWrite for u64 {
    /// Writer for u64 type 'qword' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let qword: u64 = 0xFF00000000000000;
    /// qword.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![
    ///    0x00, 0x00, 0x00, 0x00,
    ///   0x00, 0x00, 0x00, 0xFF
    /// ]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(())
    }
}

impl ByteStreamRead for i8 {
    /// Reader for i8 type 'sbyte' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03]);
    /// let mut sbyte = i8::read(&mut stream).unwrap();
    /// assert_eq!(sbyte, 0x00);
    /// sbyte = i8::read(&mut stream).unwrap();
    /// assert_eq!(sbyte, 0x01);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        let value = stream.bytes[stream.index] as i8;
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(value)
    }
}

impl ByteStreamWrite for i8 {
    /// Writer for i8 type 'sbyte' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let sbyte: i8 = 0xFF;
    /// sbyte.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        stream.bytes.push(*self as u8);
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(())
    }
}

impl ByteStreamRead for i16 {
    /// Reader for i16 type 'short' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03]);
    /// let mut short = i16::read(&mut stream).unwrap();
    /// assert_eq!(short, 0x0100);
    /// short = i16::read(&mut stream).unwrap();
    /// assert_eq!(short, 0x0302);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(2) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            i16::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1]])
        } else {
            i16::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1]])
        };
        stream.history.push((stream.index, 2));
        stream.index += 2;
        Ok(value)
    }
}

impl ByteStreamWrite for i16 {
    /// Writer for i16 type 'short' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let short: i16 = 0xFF00;
    /// short.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 2));
        stream.index += 2;
        Ok(())
    }
}

impl ByteStreamRead for i32 {
    /// Reader for i32 type 'int' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    /// let mut int = i32::read(&mut stream).unwrap();
    /// assert_eq!(int, 0x03020100);
    /// int = i32::read(&mut stream).unwrap();
    /// assert_eq!(int, 0x07060504);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            i32::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        } else {
            i32::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        };
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(value)
    }
}

impl ByteStreamWrite for i32 {
    /// Writer for i32 type 'int' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let int: i32 = 0xFF000000;
    /// int.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0x00, 0x00, 0xFF]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(())
    }
}

impl ByteStreamRead for i64 {
    /// Reader for i64 type 'long' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
    /// let mut long = i64::read(&mut stream).unwrap();
    /// assert_eq!(long, 0x0706050403020100);
    /// long = i64::read(&mut stream).unwrap();
    /// assert_eq!(long, 0x0F0E0D0C0B0A0908);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            i64::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        } else {
            i64::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        };
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(value)
    }
}

impl ByteStreamWrite for i64 {
    /// Writer for i64 type 'long' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let long: i64 = 0xFF00000000000000;
    /// long.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![
    ///    0x00, 0x00, 0x00, 0x00,
    ///   0x00, 0x00, 0x00, 0xFF
    /// ]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(())
    }
}

impl ByteStreamRead for f32 {
    /// Reader for f32 type 'float' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x80, 0x40]);
    /// let mut float = f32::read(&mut stream).unwrap();
    /// assert_eq!(float, 1.0);
    /// float = f32::read(&mut stream).unwrap();
    /// assert_eq!(float, 2.0);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(4) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            f32::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        } else {
            f32::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3]])
        };
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(value)
    }
}

impl ByteStreamWrite for f32 {
    /// Writer for f32 type 'float' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let float: f32 = 1.0;
    /// float.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0x00, 0x80, 0x3F]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 4));
        stream.index += 4;
        Ok(())
    }
}

impl ByteStreamRead for f64 {
    /// Reader for f64 type 'double' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x40]);
    /// let mut double = f64::read(&mut stream).unwrap();
    /// assert_eq!(double, 1.0);
    /// double = f64::read(&mut stream).unwrap();
    /// assert_eq!(double, 2.0);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(8) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }

        let value = if stream.endianness == Endian::Little {
            f64::from_le_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        } else {
            f64::from_be_bytes([stream.bytes[stream.index], stream.bytes[stream.index + 1], stream.bytes[stream.index + 2], stream.bytes[stream.index + 3], stream.bytes[stream.index + 4], stream.bytes[stream.index + 5], stream.bytes[stream.index + 6], stream.bytes[stream.index + 7]])
        };
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(value)
    }
}

impl ByteStreamWrite for f64 {
    /// Writer for f64 type 'double' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let double: f64 = 1.0;
    /// double.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let bytes = if stream.endianness == Endian::Little {
            self.to_le_bytes().to_vec()
        } else {
            self.to_be_bytes().to_vec()
        };
        stream.bytes.extend(bytes);
        stream.history.push((stream.index, 8));
        stream.index += 8;
        Ok(())
    }
}

impl ByteStreamRead for bool {
    /// Reader for bool type 'boolean' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00, 0x01, 0x02, 0x03]);
    /// let mut boolean = bool::read(&mut stream).unwrap();
    /// assert_eq!(boolean, false);
    /// boolean = bool::read(&mut stream).unwrap();
    /// assert_eq!(boolean, true);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        let value = stream.bytes[stream.index] != 0;
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(value)
    }
}

impl ByteStreamWrite for bool {
    /// Writer for bool type 'boolean' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let boolean: bool = true;
    /// boolean.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x01]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        stream.bytes.push(if *self { 1 } else { 0 });
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(())
    }
}

impl ByteStreamRead for char {
    /// Reader for char type 'character' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x41, 0x42, 0x43, 0x44]);
    /// let mut character = char::read(&mut stream).unwrap();
    /// assert_eq!(character, 'A');
    /// character = char::read(&mut stream).unwrap();
    /// assert_eq!(character, 'B');
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        if stream.is_out_of_bounds(1) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        let value = stream.bytes[stream.index] as char;
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(value)
    }
}

impl ByteStreamWrite for char {
    /// Writer for char type 'character' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let character: char = 'A';
    /// character.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x41]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        stream.bytes.push(*self as u8);
        stream.history.push((stream.index, 1));
        stream.index += 1;
        Ok(())
    }
}

impl ByteStreamRead for String {
    /// Reader for String type 'string' from a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to read from.
    /// * `length` - The length of the string to read.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![
    ///     0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Length
    ///     0x41, 0x42, 0x43, 0x44  // Value
    /// ]);
    /// let string = String::read(&mut stream).unwrap();
    /// assert_eq!(string, "ABCD");
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let length = u64::read(stream)? as usize;
        if stream.is_out_of_bounds(length) {
            return Err(ByteStreamError::new(stream, "Out of bounds".to_string(), ByteStreamErrorType::OutOfBounds));
        }
        let value = String::from_utf8(stream.bytes[stream.index..stream.index + length].to_vec()).map_err(|_| ByteStreamError::new(stream, "Invalid UTF-8".to_string(), ByteStreamErrorType::ReadFailure))?;
        stream.history.push((stream.index, length));
        stream.index += length;
        Ok(value)
    }
}

impl ByteStreamWrite for String {
    /// Writer for String type 'string' to a byte stream.
    /// # Arguments
    /// * `stream` - The byte stream to write to.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let string: String = "ABCD".to_string();
    /// string.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![
    ///     0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Length
    ///     0x41, 0x42, 0x43, 0x44  // Value
    /// ]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        let length = self.len() as u64;
        length.write(stream)?;
        stream.bytes.extend(self.as_bytes());
        stream.history.push((stream.index, self.len()));
        stream.index += self.len();
        Ok(())
    }
}