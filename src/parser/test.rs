#![cfg(test)]

use crate::Lexer;
use crate::Parser;
use crate::parser::*;

mod tests {

    use super::*;

    #[test]
    fn test_string_print() {
        let input = r#"println("Hello", "World");"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
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

        // Check if body[0].else_body.else_body.then_body[0] is a print
        if let Statement::If(if_statement) = ast.body[0].node.clone() {
            walk_if_chain(&if_statement);
        } else {
            panic!("Top level if invalid {:?}", ast.body[0].node);
        }
    }

    fn walk_if_chain(if_statement: &IfStatement) {
        if let Some(ref else_if_statement) = if_statement.else_body {
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
}
