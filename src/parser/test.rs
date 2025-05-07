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
        assert_eq!(ast, expected);
    }
}
