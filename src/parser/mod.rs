pub mod test;

use crate::common::token::{SpannedToken, Token};

use crate::common::ast::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<SpannedToken>,
    position: usize,
}

impl Parser {
    // Creates a new Parser instance with the provided tokens
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // Returns a reference to the current token without consuming it
    fn peek(&self) -> &Token {
        &self.tokens[self.position].node
    }

    fn peek_span(&self) -> Span {
        self.tokens[self.position].span
    }

    // Advances the parser to the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    // Parses a primary expression (e.g., literal, identifier, function call, or parenthesized expression)
    fn parse_primary(&mut self) -> SpannedExpression {
        match self.peek().clone() {
            Token::StringLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::String(value),
                })
                .spanned(self.peek_span())
            }
            Token::NumericLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(value),
                })
                .spanned(self.peek_span())
            }
            Token::BooleanLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(value),
                })
                .spanned(self.peek_span())
            }
            Token::Identifier(name) => {
                self.advance();
                if self.peek() == &Token::LeftParen {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.peek() != &Token::RightParen {
                        args.push(self.parse_expression(0));
                        while self.peek() == &Token::Comma {
                            self.advance();
                            args.push(self.parse_expression(0));
                        }
                    }
                    self.expect(Token::RightParen);
                    Expression::FunctionCall(Box::new(FunctionCall { callee: name, args }))
                        .spanned(self.peek_span())
                } else {
                    Expression::VariableRef(Box::new(VariableRef { name }))
                        .spanned(self.peek_span())
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression(0);
                self.expect(Token::RightParen);
                expr
            }
            _ => panic!("Expression token invalid {:?}", self.peek()),
        }
    }

    // Parses an expression using precedence climbing
    fn parse_expression(&mut self, min_prec: u8) -> SpannedExpression {
        let mut left = self.parse_primary();
        loop {
            let token = self.peek();
            // Early exit if it's not a binary operator
            let op = match token.get_binary_operator() {
                Some(op) => op,
                None => break,
            };

            let op_prec = op.get_precedence();
            if op_prec < min_prec {
                break;
            }

            self.advance(); // consume the operator
            // Left-associative: use op_prec + 1
            let right = self.parse_expression(op_prec + 1);
            left = Expression::Binary(Box::new(BinaryExpression {
                left,
                operator: op,
                right,
            }))
            .spanned(self.peek_span());
        }
        left
    }

    fn parse_variable(&mut self) -> SpannedStatement {
        if let Token::Identifier(name) = self.peek().clone() {
            self.advance();
            self.expect(Token::Colon);
            if let Token::Identifier(type_name) = self.peek().clone() {
                self.advance();
                self.expect(Token::Assign);
                let value = self.parse_expression(0);
                self.expect(Token::Semicolon);
                return Statement::Variable(VariableDecl {
                    name,
                    value,
                    type_name,
                })
                .spanned(self.peek_span());
            }
        }
        panic!("Invalid variable declaration");
    }

    // Parses an if statement.
    // Calls itself recursively for each else if statement.
    fn parse_if(&mut self) -> IfStatement {
        self.expect(Token::LeftParen);
        let condition = self.parse_expression(0);
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);
        let then_body = self.parse_body();
        let else_body: Option<Box<IfStatement>> =
            if self.peek() == &Token::Identifier("else".to_string()) {
                self.advance();
                if self.peek() == &Token::Identifier("if".to_string()) {
                    self.advance();
                    Some(Box::new(self.parse_if()))
                } else {
                    self.expect(Token::LeftBrace);
                    Some(Box::new(IfStatement {
                        condition: Expression::Literal(LiteralExpression {
                            value: LiteralValue::Bool(true),
                        })
                        .spanned(self.peek_span()),
                        then_body: self.parse_body(),
                        else_body: None,
                    }))
                }
            } else {
                None
            };
        IfStatement {
            condition,
            then_body,
            else_body,
        }
    }

    // Parses a statement (e.g., function call)
    fn parse_statement(&mut self) -> SpannedStatement {
        let token = self.peek().clone();
        println!("{:?}", token);
        // All statements start with an identifier
        if let Token::Identifier(name) = token.clone() {
            self.advance(); // consume identifier
            return match name.as_str() {
                "var" => self.parse_variable(),
                "if" => Statement::If(self.parse_if()).spanned(self.peek_span()),
                // "return" => self.parse_return(),
                // "while" => self.parse_while(),
                // "func" => self.parse_func(),
                _ => {
                    self.expect(Token::LeftParen);
                    let mut args = Vec::new();
                    if self.peek() != &Token::RightParen {
                        args.push(self.parse_expression(0)); // Parse first argument
                        while self.peek() == &Token::Comma {
                            self.advance(); // consume comma
                            args.push(self.parse_expression(0)); // Parse next argument
                        }
                    }
                    // Consume end of statement
                    self.expect(Token::RightParen);
                    self.expect(Token::Semicolon);
                    Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                        callee: name,
                        args,
                    })))
                    .spanned(self.peek_span())
                }
            };
        }
        panic!("Invalid token type for statement {:?}", token.clone());
    }

    // Expects a specific token and consumes it, panicking if the token doesn't match
    fn expect(&mut self, expected: Token) {
        let token = self.peek().clone();
        if token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
        self.advance(); // consume the token
    }

    // Parses the entire input and returns a vector of statements
    pub fn parse_body(&mut self) -> Vec<SpannedStatement> {
        let mut statements: Vec<SpannedStatement> = Vec::new();
        while self.peek() != &Token::EOF && self.peek() != &Token::RightBrace {
            statements.push(self.parse_statement());
        }
        if self.peek() == &Token::RightBrace {
            self.advance();
        }
        statements
    }

    // Basically just a wrapper around parse_body for now. This abstraction might be useful later
    // to add global file properties like shebangs or imports
    pub fn parse(&mut self) -> Program {
        Program {
            body: self.parse_body(),
        }
    }
}
