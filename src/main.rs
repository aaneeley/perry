mod common;
mod lexer;
use common::token::{Token, TokenWithLocation};
use lexer::tokenizer::Lexer;

fn main() {
    let input = r#"print+-*/"Hello1"123+"#;
    let mut lexer = Lexer::new(String::from(input));
    let tokens = lexer.tokenize();

    for token in tokens {
        println!(
            "{:?} at line {}, column {}",
            token.token, token.line, token.column
        );
    }
}
