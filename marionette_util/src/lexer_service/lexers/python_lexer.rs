use logos::Logos;
use serde_json::json;

#[derive(Logos, Debug, PartialEq)]
enum PythonToken {
    #[regex(r"\n", priority = 1)]
    NewLine,

    #[regex(r"(#.+)", priority = 1)]
    Comment,

    #[token("def")]
    #[token("class")]
    #[token("if")]
    #[token("else")]
    #[token("elif")]
    #[token("while")]
    #[token("for")]
    #[token("return")]
    #[token("import")]
    #[token("from")]
    #[token("as")]
    #[token("with")]
    #[token("try")]
    #[token("except")]
    #[token("finally")]
    #[token("raise")]
    #[token("assert")]
    #[token("pass")]
    #[token("break")]
    #[token("continue")]
    #[token("lambda")]
    #[token("global")]
    #[token("nonlocal")]
    #[token("del")]
    Keyword1,

    #[token("or")]
    #[token("and")]
    #[token("not")]
    #[token("is")]
    #[token("in")]
    #[token("None")]
    Keyword2,

    #[token("True")]
    #[token("False")]
    Boolean,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")]
    Number,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    String,

    #[regex(r"'[^']*'")]
    Char,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token(" +")]
    #[token("\t+")]
    Whitespace,

    #[token("(")]
    #[token(")")]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    #[token(",")]
    #[token(".")]
    #[token(":")]
    #[token(";")]
    #[token("@")]
    Punctuation,

    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("^")]
    #[token("~")]
    #[token("<>")]
    #[token("<")]
    #[token(">")]
    #[token("!")]
    #[token("=")]
    #[token(">>")]
    #[token("<<")]
    Operator
}

impl ToString for PythonToken {
    fn to_string(&self) -> String {
        match self {
            PythonToken::NewLine => "NewLine".to_string(),
            PythonToken::Comment => "Comment".to_string(),
            
            PythonToken::Keyword1 => "Keyword1".to_string(),
            PythonToken::Keyword2 => "Keyword2".to_string(),

            PythonToken::Boolean => "Boolean".to_string(),
            PythonToken::Number => "Number".to_string(),
            PythonToken::String => "String".to_string(),
            PythonToken::Char => "Char".to_string(),
            PythonToken::Identifier => "Identifier".to_string(),
            PythonToken::Whitespace => "Whitespace".to_string(),
            PythonToken::Punctuation => "Punctuation".to_string(),
            PythonToken::Operator => "Operator".to_string(),
        }
    }
}

pub struct PythonLexer {}

impl PythonLexer {
    pub fn lex(text: String) -> Result<String, String> {
        let mut tokens = PythonToken::lexer(text.as_str());
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