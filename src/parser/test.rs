#![cfg(test)]

use crate::Lexer;
use crate::Parser;
use crate::parser::*;

mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_string_print() {
        let input = r#"println("Hello", "World");"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![
            Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                callee: "println".to_string(),
                args: vec![
                    Expression::Literal(LiteralExpression {
                        value: LiteralValue::String("Hello".to_string()),
                    })
                    .spanned(Span {
                        line: 1,
                        column: 16,
                    }),
                    Expression::Literal(LiteralExpression {
                        value: LiteralValue::String("World".to_string()),
                    })
                    .spanned(Span {
                        line: 1,
                        column: 25,
                    }),
                ],
            })))
            .spanned(Span {
                line: 1,
                column: 27,
            }),
        ];
        assert_eq!(ast.unwrap().body, expected);
    }

    #[test]
    fn test_if_chaining() {
        let input = r#"
        if (1 > 2) {
            print("A");
        } else if (2 > 1) {
            print("B");
        } else {
            print("C");
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        // Check if body[0].else_body.else_body.then_body[0] is a print
        if let Statement::If(if_statement) = ast.as_ref().unwrap().body[0].node.clone() {
            walk_if_chain(&if_statement);
        } else {
            panic!("Top level if invalid {:?}", ast.unwrap().body[0].node);
        }
    }

    fn walk_if_chain(if_statement: &IfStatement) {
        if let Some(ref statement) = if_statement.else_body {
            let Statement::If(else_if_statement) = statement.as_ref().as_ref() else {
                panic!("Invalid else statement {:?}", statement.as_ref().as_ref());
            };
            walk_if_chain(else_if_statement);
            return;
        } else {
            if let Some(first_node) = if_statement.then_body.first() {
                if let Statement::Expr(expr) = first_node.node.clone() {
                    if let Expression::FunctionCall(function_call) = expr {
                        if function_call.callee == "print".to_string() {
                            return;
                        }
                    }
                };
            }
            panic!("Invalid if/else chain structure");
        }
    }

    #[test]
    fn test_loop() {
        let input = r#"
        while (true) {
            println("Hello");
        }
        "#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![
            Statement::Loop(LoopStatement {
                condition: Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(true),
                })
                .spanned(Span {
                    line: 2,
                    column: 20,
                }),
                body: vec![
                    Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                        callee: "println".to_string(),
                        args: vec![
                            Expression::Literal(LiteralExpression {
                                value: LiteralValue::String("Hello".to_string()),
                            })
                            .spanned(Span {
                                line: 3,
                                column: 28,
                            }),
                        ],
                    })))
                    .spanned(Span { line: 4, column: 9 }),
                ],
            })
            .spanned(Span { line: 5, column: 9 }),
        ];
        assert_eq!(ast.unwrap().body, expected);
    }

    #[test]
    fn test_variable() {
        let input = r#"var testvar: bool = true;testvar = false;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![
            Statement::VarDecl(VariableDecl {
                name: "testvar".to_string(),
                value: Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(true),
                })
                .spanned(Span {
                    line: 1,
                    column: 25,
                }),
                type_: Type::Bool,
            })
            .spanned(Span {
                line: 1,
                column: 32,
            }),
            Statement::VarAssignment(VariableAssignment {
                name: "testvar".to_string(),
                value: Expression::Literal(LiteralExpression {
                    value: LiteralValue::Bool(false),
                })
                .spanned(Span {
                    line: 1,
                    column: 41,
                }),
            })
            .spanned(Span {
                line: 1,
                column: 42,
            }),
        ];
        assert_eq!(ast.unwrap().body, expected);
    }

    #[test]
    fn test_function_declaration() {
        let input = r#"func name(n: int): int {return n;}"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![
            Statement::Function(FunctionDecl {
                name: "name".to_string(),
                params: vec![Parameter {
                    name: "n".to_string(),
                    type_: Type::Int,
                }],
                type_: Type::Int,
                body: vec![
                    Statement::Return(ReturnStatement {
                        value: Some(
                            Expression::VariableRef(Box::new(VariableRef {
                                name: "n".to_string(),
                            }))
                            .spanned(Span {
                                line: 1,
                                column: 33,
                            }),
                        ),
                    })
                    .spanned(Span {
                        line: 1,
                        column: 34,
                    }),
                ],
            })
            .spanned(Span {
                line: 1,
                column: 35,
            }),
        ];
        assert_eq!(ast.unwrap().body, expected);
    }

    #[test]
    fn test_function_call() {
        let input = r#"myFunction(true);"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![
            Statement::Expr(Expression::FunctionCall(Box::new(FunctionCall {
                callee: "myFunction".to_string(),
                args: vec![
                    Expression::Literal(LiteralExpression {
                        value: LiteralValue::Bool(true),
                    })
                    .spanned(Span {
                        line: 1,
                        column: 16,
                    }),
                ],
            })))
            .spanned(Span {
                line: 1,
                column: 18,
            }),
        ];
        assert_eq!(ast.unwrap().body, expected);
    }

    #[test]
    fn test_invalid_symbol() {
        let input = r#"print("Hello", "World!";"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err())
    }
}
