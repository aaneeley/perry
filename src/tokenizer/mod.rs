pub mod test;

use crate::common::{
    ast::{Span, Spannable},
    token::{BinaryOperator, SpannedToken, Token, UnaryOperator},
};

pub struct Lexer {
    input: String,
    position: usize,
    span: Span,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            span: Span { line: 1, column: 0 },
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
            self.span.line += 1;
            self.span.column = 0;
        } else {
            self.span.column += 1;
        }
        self.position += current_char.map_or(0, |c| c.len_utf8());
        current_char
    }

    // Skip over whitespace and newlines
    fn skip_empty(&mut self) {
        while let Some(ch) = self.peek_next() {
            if ch.is_whitespace() {
                self.advance(); // Skip
            } else if ch == '/' {
                if self.peek_next_n(1) == Some('/') {
                    while let Some(ch) = self.peek_next() {
                        self.advance();
                        if ch == '\n' {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> SpannedToken {
        self.skip_empty();

        let pn = self.peek_next();

        if pn.is_none() {
            return Token::EOF.spanned(Span {
                line: self.span.line,
                column: self.span.column + 1,
            });
        }

        let peeked = pn.unwrap();
        let token = match peeked {
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
                Token::Invalid(peeked.to_string())
            }
        };

        token.spanned(self.span)
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
                'a'..='z' | 'A'..='Z' | '_' => {
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

    pub fn tokenize(&mut self) -> Vec<SpannedToken> {
        let mut tokens: Vec<SpannedToken> = Vec::new();

        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token.as_ref() == &Token::EOF {
                break;
            }
        }

        tokens
    }
}
