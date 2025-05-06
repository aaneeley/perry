use crate::common::token::{Token, TokenWithLocation};

#[allow(dead_code)] // HACK:
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

#[allow(dead_code)] // HACK:
impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek_next(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let current_char = self.peek_next();
        // Update position
        if current_char == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += current_char.map_or(0, |c| c.len_utf8());
        current_char
    }

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

        let nc = self.next_char();

        if nc.is_none() {
            return TokenWithLocation {
                token: Token::EOF,
                line: self.line,
                column: self.column,
            };
        }

        let next_char = nc.unwrap();
        let token = match next_char {
            // 'a'..='z' | 'A'..='Z' | '_' => self.consume_string_literal(),
            '+' => Token::Plus,
            _ => Token::Invalid(next_char.to_string()),
        };

        TokenWithLocation {
            token,
            line: self.line,
            column: self.column,
        }
    }
    // fn consume_numeric_literal(&mut self) -> Token {}
    // fn consume_string_literal(&mut self) -> Token {}
    // fn consume_identifier(&mut self) -> Token {}
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
