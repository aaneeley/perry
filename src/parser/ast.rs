#![allow(dead_code)]

use core::panic;

use crate::common::token::{Token, TokenWithLocation};

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
pub enum BinaryOperator {
    Add,
    Subtract,
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

    pub fn peek(&self) -> &TokenWithLocation {
        &self.tokens[self.position]
    }

    pub fn peek_next(&self) -> &TokenWithLocation {
        &self.tokens[self.position + 1]
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn advance_to_semicolon(&mut self) {
        while self.peek().token != Token::Semicolon {
            self.advance();
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.peek().token.clone() {
            Token::StringLiteral(value) => {
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::String(value),
                })
            }
            Token::NumericLiteral(value) => {
                // TODO: actually parse the whole expression
                self.advance();
                Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(value),
                })
            }
            // TODO: variable ref
            // TODO: function call
            _ => panic!("Expression token invalid {:?}", self.peek().token),
        }
    }

    pub fn parse_statement(&mut self) -> Statement {
        let token = self.peek();
        // All statements start with an identifier
        if let Token::Identifier(name) = token.token.clone() {
            self.advance();
            if self.peek().token == Token::LeftParen {
                let mut args: Vec<Expression> = Vec::new();
                loop {
                    if self.peek().token == Token::RightParen {
                        break;
                    }
                    self.advance(); // Advance onto the expression 
                    args.push(self.parse_expression());
                }
                self.advance_to_semicolon();
                self.advance(); // Advance past semicolon
                return Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                    callee: name,
                    args,
                })));
            }
            panic!("Statement type not implemented");
        }
        panic!("Invalid token type for statement {:?}", token.token.clone());
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.peek().token != Token::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }
}
