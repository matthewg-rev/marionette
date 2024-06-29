use logos::Logos;
use serde_json::json;

#[derive(Logos, Debug, PartialEq)]
enum GeneralToken {
    #[regex(r"\n", priority = 1)]
    NewLine,

    #[regex(r"(;.+)", priority = 1)]
    Comment,

    #[regex(r"0x[0-9a-fA-F]+", priority = 1)]
    Address,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex("[a-zA-Z]+")]
    Text,
}

impl ToString for GeneralToken {
    fn to_string(&self) -> String {
        match self {
            GeneralToken::NewLine => "NewLine".to_string(),

            GeneralToken::Comment => "Comment".to_string(),
            GeneralToken::Address => "Address".to_string(),
            GeneralToken::Number => "Number".to_string(),
            GeneralToken::Text => "Text".to_string(),
        }
    }
}

pub struct GeneralLexer {}

impl GeneralLexer {
    pub fn lex(text: String) -> Result<String, String> {
        let mut tokens = GeneralToken::lexer(text.as_str());
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