pub mod test;

use crate::common::{
    ast::{Span, Spannable},
    token::{BinaryOperator, SpannedToken, Token, UnaryOperator},
};
use std::fmt::Display;

pub struct Tokenizer {
    input: String,
    position: usize,
    current_span: Span,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            input,
            position: 0,
            current_span: Span { line: 1, column: 0 },
        }
    }

    // Tokenizes the input and returns a vec of spanned tokens
    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, LexicalError> {
        let mut tokens: Vec<SpannedToken> = Vec::new();

        loop {
            let token = self.next_token()?;
            if token.as_ref() == &Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }

    // Consumes and returns the next token
    fn next_token(&mut self) -> Result<SpannedToken, LexicalError> {
        self.skip_empty();

        let Some(next_char) = self.peek_next() else {
            return Ok(Token::EOF.spanned(Span {
                line: self.current_span.line,
                column: self.current_span.column + 1,
            }));
        };

        let token = match next_char {
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier(),
            '0'..='9' => self.consume_numeric_literal(),
            '"' => self.consume_string_literal(),
            '=' => {
                self.advance();
                if self.peek_next() == Some('=') {
                    self.advance();
                    Token::BinaryOperator(BinaryOperator::Equal)
                } else {
                    Token::Assign
                }
            }
            '!' => {
                self.advance();
                if self.peek_next() == Some('=') {
                    self.advance();
                    Token::BinaryOperator(BinaryOperator::NotEqual)
                } else {
                    Token::UnaryOperator(UnaryOperator::Not)
                }
            }
            '<' => {
                self.advance();
                if self.peek_next() == Some('=') {
                    self.advance();
                    Token::BinaryOperator(BinaryOperator::LessThanOrEqual)
                } else {
                    Token::BinaryOperator(BinaryOperator::LessThan)
                }
            }
            '>' => {
                self.advance();
                if self.peek_next() == Some('=') {
                    self.advance();
                    Token::BinaryOperator(BinaryOperator::GreaterThanOrEqual)
                } else {
                    Token::BinaryOperator(BinaryOperator::GreaterThan)
                }
            }
            '+' => {
                self.advance();
                Token::BinaryOperator(BinaryOperator::Add)
            }
            '-' => {
                self.advance();
                Token::BinaryOperator(BinaryOperator::Subtract)
            }
            '*' => {
                self.advance();
                Token::BinaryOperator(BinaryOperator::Multiply)
            }
            '/' => {
                self.advance();
                Token::BinaryOperator(BinaryOperator::Divide)
            }
            '%' => {
                self.advance();
                Token::BinaryOperator(BinaryOperator::Modulo)
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            _ => {
                self.advance();
                return Err(LexicalError::new(
                    format!("invalid symbol: {next_char}"),
                    self.current_span,
                ));
            }
        };

        Ok(token.spanned(self.current_span))
    }

    fn consume_numeric_literal(&mut self) -> Token {
        let mut literal = String::new();
        while let Some(next) = self.peek_next() {
            match next {
                '0'..='9' => {
                    literal.push(next);
                    self.advance();
                }
                _ => break,
            }
        }
        // NOTE: Unwrap is safe here because of the char match
        Token::NumericLiteral(literal.parse().unwrap())
    }

    fn consume_string_literal(&mut self) -> Token {
        let mut literal = String::new();
        self.advance(); // Skip starting quote
        while let Some(next) = self.advance() {
            if next == '"' {
                break;
            }
            literal.push(next)
        }
        Token::StringLiteral(literal)
    }

    // Consumes an identifier or boolean literal
    fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(next) = self.peek_next() {
            match next {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    identifier.push(next);
                    self.advance();
                }
                _ => break,
            }
        }
        match identifier.as_str() {
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),
            _ => Token::Identifier(identifier),
        }
    }

    // Skip over whitespace, newlines, and comments
    fn skip_empty(&mut self) {
        while let Some(ch) = self.peek_next() {
            if ch.is_whitespace() {
                self.advance(); // Skip
            } else if ch == '/' {
                match self.peek_next_n(1) {
                    Some('/') => {
                        while let Some(ch) = self.peek_next() {
                            self.advance();
                            if ch == '\n' {
                                break;
                            }
                        }
                    }
                    Some('*') => {
                        self.advance();
                        self.advance();
                        while let Some(ch) = self.peek_next() {
                            self.advance();
                            if ch == '*' {
                                if self.peek_next() == Some('/') {
                                    self.advance();
                                    break;
                                }
                            }
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    // Peek the next char without consuming
    fn peek_next(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    // Peek the next nth char without consuming
    fn peek_next_n(&self, n: usize) -> Option<char> {
        self.input[self.position..].chars().nth(n)
    }

    // Peek and consume
    fn advance(&mut self) -> Option<char> {
        let current_char = self.peek_next();
        // Update position
        if current_char == Some('\n') {
            self.current_span.line += 1;
            self.current_span.column = 0;
        } else {
            self.current_span.column += 1;
        }
        self.position += current_char.map_or(0, |c| c.len_utf8());
        current_char
    }
}

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub message: String,
    pub span: Span,
}

impl LexicalError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexicalError: {} (at {})", self.message, self.span)
    }
}
