pub mod test;

use std::fmt::Display;

use crate::common::token::{SpannedToken, Token};

use crate::common::ast::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<SpannedToken>,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
}

impl SyntaxError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SyntaxError: {} (at {})", self.message, self.span)
    }
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

    fn peek_next(&self) -> &Token {
        &self.tokens[self.position + 1].node
    }

    fn peek_span(&self) -> Span {
        self.tokens[self.position].span
    }

    // Advances the parser to the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    // Parses a primary expression (e.g., literal, identifier, function call, or parenthesized expression)
    fn parse_primary(&mut self) -> Result<SpannedExpression, SyntaxError> {
        match self.peek().clone() {
            Token::StringLiteral(value) => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression {
                    value: LiteralValue::String(value),
                })
                .spanned(self.peek_span()))
            }
            Token::NumericLiteral(value) => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(value),
                })
                .spanned(self.peek_span()))
            }
            Token::BooleanLiteral(value) => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(value),
                })
                .spanned(self.peek_span()))
            }
            Token::Identifier(name) => {
                self.advance();
                if self.peek() == &Token::LeftParen {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if self.peek() != &Token::RightParen {
                        args.push(self.parse_expression(0)?);
                        while self.peek() == &Token::Comma {
                            self.advance();
                            args.push(self.parse_expression(0)?);
                        }
                    }
                    self.expect(Token::RightParen)?;
                    Ok(
                        Expression::FunctionCall(Box::new(FunctionCall { callee: name, args }))
                            .spanned(self.peek_span()),
                    )
                } else {
                    Ok(Expression::VariableRef(Box::new(VariableRef { name }))
                        .spanned(self.peek_span()))
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression(0)?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(SyntaxError::new(
                "Invalid symbol for primary expression".to_string(),
                self.peek_span(),
            )),
        }
    }

    // Parses an expression using precedence climbing
    fn parse_expression(&mut self, min_prec: u8) -> Result<SpannedExpression, SyntaxError> {
        let mut left = self.parse_primary()?;
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
            let right = self.parse_expression(op_prec + 1)?;
            left = Expression::Binary(Box::new(BinaryExpression {
                left,
                operator: op,
                right,
            }))
            .spanned(self.peek_span());
        }
        Ok(left)
    }

    fn parse_variable_assignemnt(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let Token::Identifier(name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::Assign)?;
        let value = self.parse_expression(0)?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::VarAssignment(VariableAssignment { name, value }).spanned(self.peek_span()))
    }

    fn parse_variable_declaration(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let Token::Identifier(name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::Colon)?;
        let Token::Identifier(type_name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected type identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::Assign)?;
        let value = self.parse_expression(0)?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::VarDecl(VariableDecl {
            name,
            value,
            type_name,
        })
        .spanned(self.peek_span()))
    }

    // Parses an if statement.
    // Calls itself recursively for each else if statement.
    fn parse_if(&mut self) -> Result<IfStatement, SyntaxError> {
        self.expect(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        let then_body = self.parse_body()?;
        let else_body: Option<Box<IfStatement>> =
            if self.peek() == &Token::Identifier("else".to_string()) {
                self.advance();
                if self.peek() == &Token::Identifier("if".to_string()) {
                    self.advance();
                    Some(Box::new(self.parse_if()?))
                } else {
                    self.expect(Token::LeftBrace)?;
                    Some(Box::new(IfStatement {
                        condition: Expression::Literal(LiteralExpression {
                            value: LiteralValue::Bool(true),
                        })
                        .spanned(self.peek_span()),
                        then_body: self.parse_body()?,
                        else_body: None,
                    }))
                }
            } else {
                None
            };
        Ok(IfStatement {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_loop(&mut self) -> Result<SpannedStatement, SyntaxError> {
        self.expect(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        let body = self.parse_body()?;
        Ok(Statement::Loop(LoopStatement { condition, body }).spanned(self.peek_span()))
    }

    fn parse_function(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let Token::Identifier(name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::LeftParen)?;
        let mut params = Vec::new();
        if self.peek() != &Token::RightParen {
            params.push(self.parse_parameter()?);
            while self.peek() == &Token::Comma {
                self.advance();
                params.push(self.parse_parameter()?);
            }
        }
        self.expect(Token::RightParen)?;
        self.expect(Token::Colon)?;
        let Token::Identifier(return_type_name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected return type identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::LeftBrace)?;
        let body = self.parse_body()?;
        Ok(Statement::Function(FunctionDecl {
            name,
            params,
            return_type_name,
            body,
        })
        .spanned(self.peek_span()))
    }

    fn parse_function_call(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let Token::Identifier(name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::LeftParen)?;
        let mut args = Vec::new();
        if self.peek() != &Token::RightParen {
            args.push(self.parse_expression(0)?); // Parse first argument
            while self.peek() == &Token::Comma {
                self.advance(); // consume comma
                args.push(self.parse_expression(0)?); // Parse next argument
            }
        }
        // Consume end of statement
        self.expect(Token::RightParen)?;
        self.expect(Token::Semicolon)?;
        Ok(
            Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                callee: name,
                args,
            })))
            .spanned(self.peek_span()),
        )
    }

    fn parse_return(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let value = self.parse_expression(0)?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::Return(ReturnStatement { value }).spanned(self.peek_span()))
    }

    // Parses a parameter
    fn parse_parameter(&mut self) -> Result<Parameter, SyntaxError> {
        let Token::Identifier(name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        self.expect(Token::Colon)?;
        let Token::Identifier(type_name) = self.peek().clone() else {
            return Err(SyntaxError::new(
                format!("expected type identifier, got {:?}", self.peek()),
                self.peek_span(),
            ));
        };
        self.advance();
        Ok(Parameter { name, type_name })
    }

    // Parses a statement (e.g., function call)
    fn parse_statement(&mut self) -> Result<SpannedStatement, SyntaxError> {
        let token = self.peek().clone();
        let Token::Identifier(name) = token.clone() else {
            return Err(SyntaxError::new(
                format!(
                    "expected identifier as first token in statement, got {:?}",
                    token
                ),
                self.peek_span(),
            ));
        };
        match name.as_str() {
            "var" => {
                self.advance();
                self.parse_variable_declaration()
            }
            "if" => {
                self.advance();
                self.parse_if()
                    .map(|if_statement| Statement::If(if_statement).spanned(self.peek_span()))
            }
            "return" => {
                self.advance();
                self.parse_return()
            }
            "while" => {
                self.advance();
                self.parse_loop()
            }
            "func" => {
                self.advance();
                self.parse_function()
            }
            _ => match self.peek_next().clone() {
                Token::LeftParen => self.parse_function_call(),
                Token::Assign => self.parse_variable_assignemnt(),
                _ => Err(SyntaxError::new(
                    format!(
                        "Expected assignment or function call after identifier, got {:?}",
                        self.peek()
                    ),
                    self.peek_span(),
                )),
            },
        }
    }

    // Expects a specific token and consumes it, panicking if the token doesn't match
    fn expect(&mut self, expected: Token) -> Result<(), SyntaxError> {
        let token = self.peek().clone();
        if token != expected {
            return Err(SyntaxError {
                message: format!(
                    "Expected {:?}, got {:?} at {:?}",
                    expected,
                    token,
                    self.peek_span()
                ),
                span: self.peek_span(),
            });
        }
        self.advance(); // consume the token
        Ok(())
    }

    // Parses the entire input and returns a vector of statements
    pub fn parse_body(&mut self) -> Result<Vec<SpannedStatement>, SyntaxError> {
        let mut statements: Vec<SpannedStatement> = Vec::new();
        while self.peek() != &Token::EOF && self.peek() != &Token::RightBrace {
            statements.push(self.parse_statement()?);
        }
        if self.peek() == &Token::RightBrace {
            self.advance();
        }
        Ok(statements)
    }

    // Basically just a wrapper around parse_body for now. This abstraction might be useful later
    // to add global file properties like shebangs or imports
    pub fn parse(&mut self) -> Result<Program, SyntaxError> {
        Ok(Program {
            body: self.parse_body()?,
        })
    }
}
