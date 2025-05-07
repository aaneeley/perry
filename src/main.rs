mod common;
mod parser;
mod tokenizer;
use std::fs::File;
use std::io::{self, Read};

use parser::ast::Parser;
use parser::ast::{self, Expression, LiteralExpression, LiteralValue};
use tokenizer::lexer::Lexer;

fn main() {
    let expression: Expression = Expression::Literal(LiteralExpression {
        value: LiteralValue::String("asdf".to_string()),
    });
    let input = read_file("test.pry").unwrap();
    println!("{}", input);
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:?}", ast);
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
