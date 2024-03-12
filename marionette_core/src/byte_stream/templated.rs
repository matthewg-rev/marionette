use crate::byte_stream::{ByteStream, ByteStreamError, ByteStreamErrorType, ByteStreamRead, ByteStreamWrite, Endian};

impl<T: ByteStreamRead> ByteStreamRead for Vec<T> {
    /// Reads a vector of a type: T which implements ByteStreamRead.
    /// The first 8 bytes of the vector is the length of the vector.
    /// The rest of the bytes are the elements of the vector.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![
    ///     0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ///     0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00
    /// ]);
    /// let vec: Vec<u32> = ByteStreamRead::read(&mut stream).unwrap();
    /// assert_eq!(vec, vec![1, 2]);
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let len = u64::read(stream)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::read(stream)?);
        }
        Ok(vec)
    }
}

impl<T: ByteStreamWrite> ByteStreamWrite for Vec<T> {
    /// Writes a vector of a type: T which implements ByteStreamWrite.
    /// The first 8 bytes of the vector is the length of the vector.
    /// The rest of the bytes are the elements of the vector.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let vec = vec![1, 2];
    /// vec.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![
    ///     0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ///     0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00
    /// ]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        (self.len() as u64).write(stream)?;
        for elem in self {
            elem.write(stream)?;
        }
        Ok(())
    }
}

impl<T: ByteStreamRead> ByteStreamRead for Option<T> {
    /// Reads an option of a type: T which implements ByteStreamRead.
    /// The first byte of the option is a boolean that indicates whether the option is Some or None.
    /// If the option is Some, the rest of the bytes are the value of the option.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamRead};
    /// let mut stream = ByteStream::new(vec![0x00]);
    /// let option: Option<u32> = ByteStreamRead::read(&mut stream).unwrap();
    /// assert_eq!(option, None);
    /// stream = ByteStream::new(vec![0x01, 0x01, 0x00, 0x00, 0x00]);
    /// let option: Option<u32> = ByteStreamRead::read(&mut stream).unwrap();
    /// assert_eq!(option, Some(1));
    /// ```
    fn read(stream: &mut ByteStream) -> Result<Self, ByteStreamError> {
        let is_some = bool::read(stream)?;
        if is_some {
            Ok(Some(T::read(stream)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: ByteStreamWrite> ByteStreamWrite for Option<T> {
    /// Writes an option of a type: T which implements ByteStreamWrite.
    /// The first byte of the option is a boolean that indicates whether the option is Some or None.
    /// If the option is Some, the rest of the bytes are the value of the option.
    /// # Examples
    /// ```
    /// use marionette_core::byte_stream::{ByteStream, ByteStreamWrite};
    /// let mut stream = ByteStream::new(Vec::new());
    /// let option: Option<i32> = None;
    /// option.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x00]);
    /// stream = ByteStream::new(Vec::new());
    /// let option: Option<i32> = Some(1);
    /// option.write(&mut stream).unwrap();
    /// assert_eq!(stream.bytes, vec![0x01, 0x01, 0x00, 0x00, 0x00]);
    /// ```
    fn write(&self, stream: &mut ByteStream) -> Result<(), ByteStreamError> {
        match self {
            Some(value) => {
                true.write(stream)?;
                value.write(stream)?;
            }
            None => {
                false.write(stream)?;
            }
        }
        Ok(())
    }
}