use logos::Logos;
use serde_json::json;

#[derive(Logos, Debug, PartialEq)]
enum LuaToken {
    #[regex(r"\n", priority = 1)]
    NewLine,

    #[regex(r"--\[", lua_comment)]
    Comment,

    #[token("function")]
    #[token("end")]
    #[token("if")]
    #[token("then")]
    #[token("else")]
    #[token("elseif")]
    #[token("for")]
    #[token("in")]
    #[token("while")]
    #[token("do")]
    #[token("repeat")]
    #[token("until")]
    #[token("return")]
    #[token("break")]
    #[token("local")]
    #[token("and")]
    #[token("or")]
    #[token("not")]
    #[token("continue")]
    #[token("goto")]
    Keyword1,

    #[token("nil")]
    #[token("true")]
    #[token("false")]
    Keyword2,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")]
    Number,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    #[regex(r"'[^']*'")]
    String,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

#[derive(Logos)]
enum LuaComment {

}

fn lua_comment(lexer: &mut logos::Lexer<'_, LuaToken>) -> Result<(), ()> {
    
}

impl ToString for LuaToken {
    fn to_string(&self) -> String {
        match self {
            LuaToken::NewLine => "NewLine".to_string(),
            LuaToken::Comment => "Comment".to_string(),

            LuaToken::Keyword1 => "Keyword1".to_string(),
            LuaToken::Keyword2 => "Keyword2".to_string(),

            LuaToken::Number => "Number".to_string(),
            LuaToken::String => "String".to_string(),

            LuaToken::Identifier => "Identifier".to_string(),
        }
    }
}

pub struct LuaLexer {}

impl LuaLexer {
    pub fn lex(text: String) -> Result<String, String> {
        let mut tokens = LuaToken::lexer(text.as_str());
        let mut v: Vec<serde_json::Value> = Vec::new();
        
        while let Some(token) = tokens.next() {
            if token.is_err() { continue; }
            let token = token.unwrap();
            
            v.push(json!({
                "slice": tokens.slice(),
                "span": tokens.span(),
                "token": token.to_string()
            }));
        }

        Ok(json!(v).to_string())
    }
}