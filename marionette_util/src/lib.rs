pub mod lexer_service;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_lexer_test() {
        let lua_code = r#"
            local a = 1
            local b = 2
            local c = 3
            print(a + b + c)"#;
        let tokens = lexer_service::lexers::lua_lexer::LuaLexer::lex(lua_code.to_string());
        assert_eq!(tokens.is_ok(), true);
        assert_eq!(tokens.unwrap().len(), 1078);
    }

    #[test]
    fn python_lexer_test() {
        let python_code = r#"
            a = 1
            b = 2
            c = 3
            print(a + b + c)"#;
        let tokens = lexer_service::lexers::python_lexer::PythonLexer::lex(python_code.to_string());
        assert_eq!(tokens.is_ok(), true);
        assert_eq!(tokens.unwrap().len(), 1325);
    }

    #[test]
    fn general_lexer_test() {
        let general_text = r#"
            0x1     $KW1{GETGLOBAL} 0 -1    ; print
            0x2     $KW1{LOADK}     1 -2    ; "Hello, World!"
            0x3     $KW1{CALL}      0 2 1
            0x4     $KW2{RETURN}    0 1"#;
        let tokens = lexer_service::lexers::general_lexer::GeneralLexer::lex(general_text.to_string());
        assert_eq!(tokens.is_ok(), true);
        assert_eq!(tokens.unwrap().len(), 507);
    }
}
