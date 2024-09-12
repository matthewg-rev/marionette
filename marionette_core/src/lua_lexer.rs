use logos::Logos;
use serde_json::json;

#[derive(Logos, Debug, PartialEq)]
enum LuaToken {
    #[regex(r"\n", priority = 1)]
    NewLine,

    #[regex(r"--", lua_comment, priority = 1)]
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
    #[regex(r"\[=*\[", lua_string)]
    String,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

#[derive(Logos, Debug, PartialEq)]
enum LuaMultiLineString {
    #[token("[", priority = 10)]
    Open,
    #[token("]", priority = 10)]
    Close,
    #[token("=", priority = 10)]
    Equals,
    #[regex(".")]
    Content
}

fn lua_string(lex: &mut logos::Lexer<LuaToken>) -> Result<(), ()> {
    let remainder = lex.remainder();
    let mut chars = remainder.chars().peekable();

    // get currently matched data
    let data = lex.slice().to_string();
    let eq_count = data.len() - 2; // remove opening brackets from length

    println!("remainder: {:?}", remainder);

    // define the closing sequence
    let closing_sequence = format!("]{}]", "=".repeat(eq_count));

    // push characters into content until closing sequence is found
    let mut content = String::new();
    while let Some(c) = chars.next() {
        content.push(c);
        if content.ends_with(&closing_sequence) {
            break;
        }
    }

    // check closing sequence
    if !content.ends_with(&closing_sequence) {
        return Err(());
    }

    // bump the lexer
    let comment_len = remainder.len() - chars.collect::<String>().len();
    lex.bump(comment_len);

    return Ok(());
}

#[derive(Logos, Debug, PartialEq)]
enum LuaComment {
    #[token("[", priority = 10)]
    Open,
    #[token("]", priority = 10)]
    Close,
    #[token("=", priority = 10)]
    Equals,
    #[regex(".")]
    Content
}

fn lua_comment(lex: &mut logos::Lexer<LuaToken>) -> Result<(), ()> {
    let remainder = lex.remainder();
    let mut chars = remainder.chars().peekable();
    let mut multi_line = false;

    if chars.peek() == Some(&'[') {
        chars.next();
        multi_line = true;
    }

    println!("remainder: {:?} | multi_line: {:?}", remainder, multi_line);

    if multi_line {
        // count the number of equals signs
        let mut eq_count = 0;
        while let Some('=') = chars.peek() {
            chars.next();
            eq_count += 1;
        }

        // make sure the enter bracket is there
        if chars.next() != Some('[') {
            return Err(());
        }

        // define the closing sequence
        let closing_sequence = format!("]{}]", "=".repeat(eq_count));

        // push characters into content until closing sequence is found
        let mut content = String::new();
        while let Some(c) = chars.next() {
            content.push(c);
            if content.ends_with(&closing_sequence) {
                break;
            }
        }

        // check closing sequence
        if !content.ends_with(&closing_sequence) {
            return Err(());
        }

        // bump the lexer
        let comment_len = remainder.len() - chars.collect::<String>().len();
        lex.bump(comment_len);

        return Ok(());
    }

    // Single-line comment
    while let Some(c) = chars.peek() {
        if c == &'\n' {
            break;
        }
        chars.next();
    }

    let comment_len = remainder.len() - chars.collect::<String>().len();
    lex.bump(comment_len);

    return Ok(())
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