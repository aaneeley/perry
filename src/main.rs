mod analyzer;
mod common;
mod parser;
mod tokenizer;
use std::env;
use std::fs::File;
use std::io::{self, Read};

use analyzer::Analyzer;
use parser::Parser;
use tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = read_file(&args[1])?;
    let mut tokenizer = Tokenizer::new(input.to_string());
    let tokens = tokenizer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut analyzer = Analyzer::new(&ast);
    let result = analyzer.analyze();
    println!("{:#?}", result);

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
