use logos::{Logos, Lexer};

fn identifier(lex: &mut Lexer<LuaToken>) -> Option<String> {
    let mut identifier = String::new();
    identifier.insert_str(0, lex.slice());
    Some(identifier)
}

fn sliced_data(lex: &mut Lexer<LuaToken>) -> Option<String> {
    Some(lex.slice().to_string())
}

#[derive(Logos, Debug, PartialEq)]
pub enum LuaToken {
    #[regex("\\s+")]
    Whitespace,

    #[regex("[_a-zA-Z][a-zA-Z0-9]*", priority=3, callback = identifier)]
    Identifier(String),

    #[regex("--[^\\[].*", priority = 3, callback = sliced_data)]
    SingleComment(String),
    #[regex("--\\[(=*)\\[(.|\n)*?\\](=*)\\]", priority = 2, callback = sliced_data)]
    LongCommentStart(String),

    #[token("+")]
    SymAdd,
    #[token("-")]
    SymSub,
    #[token("*")]
    SymMultiply,
    #[token("/")]
    SymDivide,
    #[token("%")]
    SymModulus,
    #[token("^")]
    SymPow,
    #[token("#")]
    SymLen,
    #[token("==")]
    SymEq,
    #[token("~=")]
    SymNeq,
    #[token("<=")]
    SymLte,
    #[token(">=")]
    SymGte,
    #[token("<")]
    SymLt,
    #[token(">")]
    SymGt,
    #[token("=")]
    SymAssign,
    #[token("(")]
    SymLeftParantheses,
    #[token(")")]
    SymRightParantheses,
    #[token("{")]
    SymLeftCurlyBrace,
    #[token("}")]
    SymRightCurlyBrace,
    #[token("[")]
    SymLeftBracket,
    #[regex("(\\[=*\\[)", callback = sliced_data)]
    LongLiteralStart(String),
    #[token("]")]
    SymRightBracket,
    #[regex("(\\]=*\\])", callback = sliced_data)]
    LongLiteralEnd(String),
    #[token(";")]
    SymSemicolon,
    #[token(":")]
    SymColon,
    #[token(",")]
    SymComma,
    #[token(".")]
    SymPeriod,
    #[token("..")]
    SymConcat,
    #[token("...")]
    SymVararg,

    #[token("and")]
    KwAnd,
    #[token("break")]
    KwBreak,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("elseif")]
    KwElseIf,
    #[token("end")]
    KwEnd,
    #[token("false")]
    KwFalse,
    #[token("for")]
    KwFor,
    #[token("function")]
    KwFunction,
    #[token("if")]
    KwIf,
    #[token("in")]
    KwIn,
    #[token("local")]
    KwLocal,
    #[token("nil")]
    KwNil,
    #[token("not")]
    KwNot,
    #[token("or")]
    KwOr,
    #[token("repeat")]
    KwRepeat,
    #[token("return")]
    KwReturn,
    #[token("then")]
    KwThen,
    #[token("true")]
    KwTrue,
    #[token("until")]
    KwUntil, 
    #[token("while")]
    KwWhile,

    #[regex("\'(\\.|[^\'])*\'", callback = sliced_data)]
    ShortString(String),
    #[regex("\"(\\.|[^\"])*\"", callback = sliced_data)]
    String(String),
    LongLiteral(String),
    #[regex("-?[0-9]+", priority = 2, callback = sliced_data)]
    Integer(String),
    #[regex("0x[0-9a-fA-F]+", callback = sliced_data)]
    Hexadecimal(String),
    #[regex("-?\\d+(\\.\\d*)?[eE][-+]?\\d+", callback = sliced_data)]
    Exponent(String),
    #[regex("-?[0-9]+\\.[0-9]+", callback = sliced_data)]
    Number(String),

    EndOfFile,
}

pub struct LuaLexer {}
impl LuaLexer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn lexer(&self, code: &str) -> Vec<Result<LuaToken, ()>> {
        let mut tokens = Vec::new();
        let mut lexer = LuaToken::lexer(code);
        while let Some(token) = lexer.next() {
            tokens.push(token);
        }
        tokens
    }
}