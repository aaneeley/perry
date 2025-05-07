use crate::common::token::{BinaryOperator, Token, TokenWithLocation};

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 0,
        }
    }

    // Peek the next char without consuming
    fn peek_next(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    // Peek and consume
    fn next_char(&mut self) -> Option<char> {
        let current_char = self.peek_next();
        // Update position
        if current_char == Some('\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.position += current_char.map_or(0, |c| c.len_utf8());
        current_char
    }

    // Skip over whitespace and newlines
    // TODO: Skip comments
    fn skip_empty(&mut self) {
        while let Some(ch) = self.peek_next() {
            if ch.is_whitespace() {
                self.next_char(); // Skip
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> TokenWithLocation {
        self.skip_empty();

        let pn = self.peek_next();

        if pn.is_none() {
            return TokenWithLocation {
                token: Token::EOF,
                line: self.line,
                column: self.column + 1,
            };
        }

        let peeked = pn.unwrap();
        let token = match peeked {
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier(),
            '0'..='9' => self.consume_numeric_literal(),
            '"' => self.consume_string_literal(),
            '+' => {
                self.next_char();
                Token::BinaryOperator(BinaryOperator::Add)
            }
            '-' => {
                self.next_char();
                Token::BinaryOperator(BinaryOperator::Subtract)
            }
            '*' => {
                self.next_char();
                Token::BinaryOperator(BinaryOperator::Multiply)
            }
            '/' => {
                self.next_char();
                Token::BinaryOperator(BinaryOperator::Divide)
            }
            '%' => {
                self.next_char();
                Token::BinaryOperator(BinaryOperator::Modulo)
            }
            ';' => {
                self.next_char();
                Token::Semicolon
            }
            ',' => {
                self.next_char();
                Token::Comma
            }
            '(' => {
                self.next_char();
                Token::LeftParen
            }
            ')' => {
                self.next_char();
                Token::RightParen
            }
            _ => {
                self.next_char();
                Token::Invalid(peeked.to_string())
            }
        };

        token.with_location(self.line, self.column)
    }

    fn consume_numeric_literal(&mut self) -> Token {
        let mut literal = String::new();
        while let Some(next) = self.peek_next() {
            match next {
                '0'..='9' => {
                    literal.push(next);
                    self.next_char();
                }
                _ => break,
            }
        }
        // NOTE: Unwrap is safe here because of the char match
        Token::NumericLiteral(literal.parse().unwrap())
    }

    fn consume_string_literal(&mut self) -> Token {
        let mut literal = String::new();
        self.next_char(); // Skip starting quote
        while let Some(next) = self.next_char() {
            if next == '"' {
                break;
            }
            literal.push(next)
        }
        Token::StringLiteral(literal)
    }

    fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(next) = self.peek_next() {
            match next {
                'a'..='z' | 'A'..='Z' | '_' => {
                    identifier.push(next);
                    self.next_char();
                }
                _ => break,
            }
        }
        Token::Identifier(identifier)
    }

    pub fn tokenize(&mut self) -> Vec<TokenWithLocation> {
        let mut tokens: Vec<TokenWithLocation> = Vec::new();

        loop {
            let token_with_location = self.next_token();
            tokens.push(token_with_location.clone());
            if token_with_location.token == Token::EOF {
                break;
            }
        }

        tokens
    }
}
