#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)] // HACK:
pub enum Token {
    Identifier(String),
    NumericLiteral(i32),
    StringLiteral(String),
    LogicalLiteral(bool),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    LeftParen,
    RightParen,
    Semicolon,
    EOF,
    Invalid(String),
}

#[derive(Debug, Clone)]
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
}
