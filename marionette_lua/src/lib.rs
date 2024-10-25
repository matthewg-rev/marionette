pub mod lua_binary;
pub mod cfg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_deserialization_tests() {
        let raw_file = vec![
            0x1b, 0x4c, 0x75, 0x61, 0x51, 0x00, 0x01, 0x04, 0x04, 0x04, 0x08, 0x00,
            0x18, 0x00, 0x00, 0x00, 0x40, 0x2f, 0x64, 0x65, 0x76, 0x2f, 0x73, 0x68,
            0x6d, 0x2f, 0x6c, 0x75, 0x61, 0x63, 0x2e, 0x6e, 0x6c, 0x73, 0x71, 0x7a,
            0x71, 0x67, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x02, 0x06, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
            0x17, 0x40, 0x40, 0x00, 0x16, 0x00, 0x00, 0x80, 0x02, 0x40, 0x00, 0x00,
            0x02, 0x00, 0x80, 0x00, 0x1e, 0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x04, 0x03, 0x00, 0x00, 0x00, 0x67, 0x67, 0x00, 0x03, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x40, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x61, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        let mut stream = ByteStream::new(raw_file.clone());
        let result = lua_binary::LuaBinary::read(&mut stream);
        assert!(result.is_ok());
        let result = result.unwrap();
        let binary = result.clone();

        let mut stream = ByteStream::new(vec![]);
        let result = lua_binary::LuaBinary::write(&result, &mut stream);
        assert!(result.is_ok());
        assert_eq!(stream.remaining(), raw_file);

        for function in binary.functions {
            let graph = cfg::get_graph(function.clone());
        }
    }
}
