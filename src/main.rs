mod analyzer;
mod common;
mod interpreter;
mod parser;
mod tokenizer;
use std::env;
use std::fs::File;
use std::io::{self, Read};

use analyzer::Analyzer;
use interpreter::Interpreter;
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
    let tokens = match tokenizer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let mut analyzer = Analyzer::new(&ast);
    if let Err(err) = analyzer.analyze() {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    let mut interpreter = Interpreter::new(&ast);
    if let Err(err) = interpreter.execute() {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
