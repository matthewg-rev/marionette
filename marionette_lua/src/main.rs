#![feature(build_hasher_simple_hash_one)]
use std::env;

pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let test_code = vec![
            include_str!("testing/lexer/keywords.lua"),
            include_str!("testing/lexer/numbers.lua"),
            include_str!("testing/lexer/strings.lua"),
            include_str!("testing/lexer/symbols.lua"),
            include_str!("testing/lexer/comments.lua"),
        ];

        let lexer = lexer::LuaLexer::new();
        for code in test_code {
            let tokens = lexer.lexer(code);
            println!("{:?}", tokens);
            println!("-----------------------------");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}