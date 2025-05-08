mod common;
mod interpreter;
mod parser;
mod tokenizer;
use std::env;
use std::fs::File;
use std::io::{self, Read};

use parser::Parser;
use tokenizer::Lexer;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = read_file(&args[1])?;
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    for statement in ast.unwrap().body {
        println!("{:#?}", statement);
    }

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
