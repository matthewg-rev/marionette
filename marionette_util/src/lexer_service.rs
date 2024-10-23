pub mod lexers;
use crate::lexer_service::lexers::*;

pub struct LexerService {}

impl LexerService {
    pub fn lex(code: String, lexer: String) -> Result<String, String> {
        if lexer == "general" {
            let tokens = general_lexer::GeneralLexer::lex(code);
            if tokens.is_err() {
                return Err(tokens.err().unwrap().to_string());
            }
            return Ok(tokens.unwrap());
        } else if lexer == "python" {
            let tokens = python_lexer::PythonLexer::lex(code);
            if tokens.is_err() {
                return Err(tokens.err().unwrap().to_string());
            }
            return Ok(tokens.unwrap());
        } else if lexer == "lua" {
            let tokens = lua_lexer::LuaLexer::lex(code);
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