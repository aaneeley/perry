pub mod test;

use std::ops::RangeInclusive;

use crate::common::token::{Token, TokenWithLocation};

use crate::common::ast::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<TokenWithLocation>,
    position: usize,
}

impl Parser {
    // Creates a new Parser instance with the provided tokens
    pub fn new(tokens: Vec<TokenWithLocation>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // Returns a reference to the current token without consuming it
    fn peek(&self) -> &TokenWithLocation {
        &self.tokens[self.position]
    }

    // Advances the parser to the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    // Parses a primary expression (e.g., literal, identifier, function call, or parenthesized expression)
    fn parse_primary(&mut self) -> Expression {
        match self.peek().token.clone() {
            Token::StringLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::String(value),
                })
            }
            Token::NumericLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(value),
                })
            }
            Token::BooleanLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(value),
                })
            }
            Token::Identifier(name) => {
                self.advance();
                if self.peek().token == Token::LeftParen {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.peek().token != Token::RightParen {
                        args.push(self.parse_expression(0));
                        while self.peek().token == Token::Comma {
                            self.advance();
                            args.push(self.parse_expression(0));
                        }
                    }
                    self.expect(Token::RightParen);
                    Expression::FunctionCall(Box::new(FunctionCall { callee: name, args }))
                } else {
                    Expression::VariableRef(Box::new(VariableRef { name }))
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression(0);
                self.expect(Token::RightParen);
                expr
            }
            _ => panic!("Expression token invalid {:?}", self.peek().token),
        }
    }

    // Parses an expression using precedence climbing
    fn parse_expression(&mut self, min_prec: u8) -> Expression {
        let mut left = self.parse_primary();
        loop {
            let token = self.peek();
            // Early exit if it's not a binary operator
            let op = match token.token.get_binary_operator() {
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
            }));
        }
        left
    }

    fn parse_variable(&mut self) -> Statement {
        if let Token::Identifier(name) = self.peek().token.clone() {
            self.advance();
            self.expect(Token::Colon);
            if let Token::Identifier(type_name) = self.peek().token.clone() {
                self.advance();
                self.expect(Token::Assign);
                let value = self.parse_expression(0);
                self.expect(Token::Semicolon);
                return Statement::Variable(VariableDecl {
                    name,
                    value,
                    type_name,
                });
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
            if self.peek().token == Token::Identifier("else".to_string()) {
                self.advance();
                if self.peek().token == Token::Identifier("if".to_string()) {
                    self.advance();
                    Some(Box::new(self.parse_if()))
                } else {
                    self.expect(Token::LeftBrace);
                    Some(Box::new(IfStatement {
                        condition: Expression::Literal(LiteralExpression {
                            value: LiteralValue::Bool(true),
                        }),
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
    fn parse_statement(&mut self) -> Statement {
        let token = self.peek().clone();
        println!("{:?}", token);
        // All statements start with an identifier
        if let Token::Identifier(name) = token.token.clone() {
            self.advance(); // consume identifier
            return match name.as_str() {
                "var" => self.parse_variable(),
                "if" => Statement::If(self.parse_if()),
                // "return" => self.parse_return(),
                // "while" => self.parse_while(),
                // "func" => self.parse_func(),
                _ => {
                    self.expect(Token::LeftParen);
                    let mut args = Vec::new();
                    if self.peek().token != Token::RightParen {
                        args.push(self.parse_expression(0)); // Parse first argument
                        while self.peek().token == Token::Comma {
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
                }
            };
        }
        panic!("Invalid token type for statement {:?}", token.token.clone());
    }

    // Expects a specific token and consumes it, panicking if the token doesn't match
    fn expect(&mut self, expected: Token) {
        let token = self.peek().token.clone();
        if token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
        self.advance(); // consume the token
    }

    // Parses the entire input and returns a vector of statements
    pub fn parse_body(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.peek().token != Token::EOF && self.peek().token != Token::RightBrace {
            statements.push(self.parse_statement());
        }
        if self.peek().token == Token::RightBrace {
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
