use serde_json::json;
use crate::general_lexer::GeneralLexer;
use crate::python_lexer::PythonLexer;
use crate::lua_lexer::LuaLexer;

pub struct LexerService {}

impl LexerService {
    pub fn lex(code: String, lexer: String) -> Result<String, String> {
        if lexer == "general" {
            let tokens = GeneralLexer::lex(code);
            if tokens.is_err() {
                return Err(tokens.err().unwrap().to_string());
            }
            return Ok(tokens.unwrap());
        } else if lexer == "python" {
            let tokens = PythonLexer::lex(code);
            if tokens.is_err() {
                return Err(tokens.err().unwrap().to_string());
            }
            return Ok(tokens.unwrap());
        } else if lexer == "lua" {
            let tokens = LuaLexer::lex(code);
            if tokens.is_err() {
                return Err(tokens.err().unwrap().to_string());
            }
            return Ok(tokens.unwrap());
        } else {
            let mut v: Vec<serde_json::Value> = Vec::new();
            let json = serde_json::to_string(&v);
            if json.is_err() {
                return Err(json.err().unwrap().to_string());
            }
            return Ok(json.unwrap());
        }
    }
}