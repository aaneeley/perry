#![cfg(test)]

use crate::Lexer;
use crate::Parser;
use crate::parser::*;

mod tests {
    use super::*;
    use crate::common::token::BinaryOperator;

    #[test]
    fn test_string_print() {
        let input = r#"println("Hello", "World");"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let expected = vec![Statement::Expr(Expression::FunctionCall(Box::new(
            FunctionCall {
                callee: "println".to_string(),
                args: vec![
                    Expression::Literal(LiteralExpression {
                        value: LiteralValue::String("Hello".to_string()),
                    }),
                    Expression::Literal(LiteralExpression {
                        value: LiteralValue::String("World".to_string()),
                    }),
                ],
            },
        )))];
        assert_eq!(ast.body, expected);
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
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        // This is a horrendous black hole of paren matching hell
        // The test works and that's all that matters
        let expected = vec![Statement::If(IfStatement {
            condition: Expression::Binary(Box::new(BinaryExpression {
                left: Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(1),
                }),
                right: Expression::Literal(LiteralExpression {
                    value: LiteralValue::Number(2),
                }),
                operator: BinaryOperator::GreaterThan,
            })),
            then_body: vec![Statement::Expr(Expression::FunctionCall(Box::new(
                FunctionCall {
                    callee: "print".to_string(),
                    args: vec![Expression::Literal(LiteralExpression {
                        value: LiteralValue::String("A".to_string()),
                    })],
                },
            )))],
            else_body: Some(Box::new(IfStatement {
                condition: Expression::Binary(Box::new(BinaryExpression {
                    left: Expression::Literal(LiteralExpression {
                        value: LiteralValue::Number(2),
                    }),
                    right: Expression::Literal(LiteralExpression {
                        value: LiteralValue::Number(1),
                    }),
                    operator: BinaryOperator::GreaterThan,
                })),
                then_body: vec![Statement::Expr(Expression::FunctionCall(Box::new(
                    FunctionCall {
                        callee: "print".to_string(),
                        args: vec![Expression::Literal(LiteralExpression {
                            value: LiteralValue::String("B".to_string()),
                        })],
                    },
                )))],
                else_body: Some(Box::new(IfStatement {
                    condition: Expression::Literal(LiteralExpression {
                        value: LiteralValue::Bool(true),
                    }),
                    then_body: vec![Statement::Expr(Expression::FunctionCall(Box::new(
                        FunctionCall {
                            callee: "print".to_string(),
                            args: vec![Expression::Literal(LiteralExpression {
                                value: LiteralValue::String("C".to_string()),
                            })],
                        },
                    )))],
                    else_body: None,
                })),
            })),
        })];
        assert_eq!(ast.body, expected);
    }
}
