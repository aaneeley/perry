#![allow(dead_code, unused_variables)]

use core::panic;

use crate::common::token::{BinaryOperator, Token, TokenWithLocation};

#[derive(Debug)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Literal(LiteralExpression),
    FunctionCall(Box<FunctionCall>),
    VariableRef(Box<VariableRef>),
}

#[derive(Debug)]
pub enum Statement {
    Function(FunctionDecl),
    Variable(VariableDecl),
    If(IfStatement),
    Return(ReturnStatement),
    Expr(Expression), // To allow void expressions in function body
}

#[derive(Debug)]
pub struct VariableDecl {
    name: String,
    value: Expression,
}

#[derive(Debug)]
pub struct FunctionDecl {
    name: String,
    params: Vec<String>, // TODO: Make this a struct with type
    body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionCall {
    callee: String,
    args: Vec<Expression>,
}

#[derive(Debug)]
pub struct VariableRef {
    name: String,
}

#[derive(Debug)]
pub struct BinaryExpression {
    left: Expression,
    right: Expression,
    operator: BinaryOperator,
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub value: LiteralValue,
}

#[derive(Debug)]
pub enum LiteralValue {
    String(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Expression,
    then_body: Vec<Statement>,
    else_body: Vec<Statement>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    value: Expression,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<TokenWithLocation>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithLocation>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> &TokenWithLocation {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

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

    fn parse_statement(&mut self) -> Statement {
        let token = self.peek();
        // All statements start with an identifier
        if let Token::Identifier(name) = token.token.clone() {
            self.advance(); // consume identifier
            if self.peek().token == Token::LeftParen {
                // Function call statement
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
                return Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                    callee: name,
                    args,
                })));
            }
            panic!("Statement type not implemented");
        }
        panic!("Invalid token type for statement {:?}", token.token.clone());
    }

    // Helper method to expect a specific token and consume it
    fn expect(&mut self, expected: Token) {
        let token = self.peek().token.clone();
        if token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
        self.advance(); // consume the token
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.peek().token != Token::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }
}
