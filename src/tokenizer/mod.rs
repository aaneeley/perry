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

    // Peek and consume
    fn next_char(&mut self) -> Option<char> {
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

    fn next_token(&mut self) -> SpannedToken {
        self.skip_empty();

        let pn = self.peek_next();

        if pn.is_none() {
            return Token::EOF.spanned(self.span);
        }

        let peeked = pn.unwrap();
        let token = match peeked {
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier(),
            '0'..='9' => self.consume_numeric_literal(),
            '"' => self.consume_string_literal(),
            '=' => {
                self.next_char();
                if self.peek_next() == Some('=') {
                    self.next_char();
                    Token::BinaryOperator(BinaryOperator::Equal)
                } else {
                    Token::Assign
                }
            }
            '!' => {
                self.next_char();
                if self.peek_next() == Some('=') {
                    self.next_char();
                    Token::BinaryOperator(BinaryOperator::NotEqual)
                } else {
                    Token::UnaryOperator(UnaryOperator::Not)
                }
            }
            '<' => {
                self.next_char();
                if self.peek_next() == Some('=') {
                    self.next_char();
                    Token::BinaryOperator(BinaryOperator::LessThanOrEqual)
                } else {
                    Token::BinaryOperator(BinaryOperator::LessThan)
                }
            }
            '>' => {
                self.next_char();
                if self.peek_next() == Some('=') {
                    self.next_char();
                    Token::BinaryOperator(BinaryOperator::GreaterThanOrEqual)
                } else {
                    Token::BinaryOperator(BinaryOperator::GreaterThan)
                }
            }
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
            ':' => {
                self.next_char();
                Token::Colon
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
            '{' => {
                self.next_char();
                Token::LeftBrace
            }
            '}' => {
                self.next_char();
                Token::RightBrace
            }
            _ => {
                self.next_char();
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

    // Consumes an identifier or boolean literal
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
