use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)] // HACK:
pub enum Token {
    Identifier(String),
    NumericLiteral(i32),
    StringLiteral(String),
    LogicalLiteral(bool),
    BinaryOperator(BinaryOperator),
    Comma,
    Equal,
    LeftParen,
    RightParen,
    Semicolon,
    EOF,
    Invalid(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Clone, Debug)]
pub struct TokenWithLocation {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn with_location(self, line: usize, column: usize) -> TokenWithLocation {
        TokenWithLocation {
            token: self,
            line,
            column,
        }
    }

    pub fn get_binary_operator(&self) -> Option<BinaryOperator> {
        match self {
            Token::BinaryOperator(op) => Some(*op),
            _ => None,
        }
    }
}

impl BinaryOperator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperator::Add => 9,
            BinaryOperator::Subtract => 9,
            BinaryOperator::Multiply => 10,
            BinaryOperator::Divide => 10,
            BinaryOperator::Modulo => 10,
        }
    }
}
