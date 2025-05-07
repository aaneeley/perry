mod common;
mod parser;
mod tokenizer;
use std::fs::File;
use std::io::{self, Read};

use parser::Parser;
use tokenizer::Lexer;

fn main() {
    let input = read_file("test.pry").unwrap();
    println!("INPUT FILE:\n{}\n", input);
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for statement in ast.body {
        println!("{:#?}", statement);
    }
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
