#![cfg(test)]

use super::*;

mod tests {
    use super::*;

    #[test]
    fn test_simple_print() {
        let input = r#"print("Hello", "World!");"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            TokenWithLocation {
                token: Token::Identifier("print".to_string()),
                line: 1,
                column: 5,
            },
            TokenWithLocation {
                token: Token::LeftParen,
                line: 1,
                column: 6,
            },
            TokenWithLocation {
                token: Token::StringLiteral("Hello".to_string()),
                line: 1,
                column: 13,
            },
            TokenWithLocation {
                token: Token::Comma,
                line: 1,
                column: 14,
            },
            TokenWithLocation {
                token: Token::StringLiteral("World!".to_string()),
                line: 1,
                column: 23,
            },
            TokenWithLocation {
                token: Token::RightParen,
                line: 1,
                column: 24,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 25,
            },
            TokenWithLocation {
                token: Token::EOF,
                line: 1,
                column: 26,
            },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_artihmetic_print() {
        let input = r#"println((1 + 2) * 3 / 4);"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            TokenWithLocation {
                token: Token::Identifier("println".to_string()),
                line: 1,
                column: 7,
            },
            TokenWithLocation {
                token: Token::LeftParen,
                line: 1,
                column: 8,
            },
            TokenWithLocation {
                token: Token::LeftParen,
                line: 1,
                column: 9,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(1),
                line: 1,
                column: 10,
            },
            TokenWithLocation {
                token: Token::BinaryOperator(BinaryOperator::Add),
                line: 1,
                column: 12,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(2),
                line: 1,
                column: 14,
            },
            TokenWithLocation {
                token: Token::RightParen,
                line: 1,
                column: 15,
            },
            TokenWithLocation {
                token: Token::BinaryOperator(BinaryOperator::Multiply),
                line: 1,
                column: 17,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(3),
                line: 1,
                column: 19,
            },
            TokenWithLocation {
                token: Token::BinaryOperator(BinaryOperator::Divide),
                line: 1,
                column: 21,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(4),
                line: 1,
                column: 23,
            },
            TokenWithLocation {
                token: Token::RightParen,
                line: 1,
                column: 24,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 25,
            },
            TokenWithLocation {
                token: Token::EOF,
                line: 1,
                column: 26,
            },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_arithmetic_bool_variable() {
        let input = r#"var testvar: bool = (5 >= (4 - 3));"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            TokenWithLocation {
                token: Token::Identifier("var".to_string()),
                line: 1,
                column: 3,
            },
            TokenWithLocation {
                token: Token::Identifier("testvar".to_string()),
                line: 1,
                column: 11,
            },
            TokenWithLocation {
                token: Token::Colon,
                line: 1,
                column: 12,
            },
            TokenWithLocation {
                token: Token::Identifier("bool".to_string()),
                line: 1,
                column: 17,
            },
            TokenWithLocation {
                token: Token::Assign,
                line: 1,
                column: 19,
            },
            TokenWithLocation {
                token: Token::LeftParen,
                line: 1,
                column: 21,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(5),
                line: 1,
                column: 22,
            },
            TokenWithLocation {
                token: Token::BinaryOperator(BinaryOperator::GreaterThanOrEqual),
                line: 1,
                column: 25,
            },
            TokenWithLocation {
                token: Token::LeftParen,
                line: 1,
                column: 27,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(4),
                line: 1,
                column: 28,
            },
            TokenWithLocation {
                token: Token::BinaryOperator(BinaryOperator::Subtract),
                line: 1,
                column: 30,
            },
            TokenWithLocation {
                token: Token::NumericLiteral(3),
                line: 1,
                column: 32,
            },
            TokenWithLocation {
                token: Token::RightParen,
                line: 1,
                column: 33,
            },
            TokenWithLocation {
                token: Token::RightParen,
                line: 1,
                column: 34,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 35,
            },
            TokenWithLocation {
                token: Token::EOF,
                line: 1,
                column: 36,
            },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_variable_reference() {
        let input = r#"var testvar: bool = testvar;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            TokenWithLocation {
                token: Token::Identifier("var".to_string()),
                line: 1,
                column: 3,
            },
            TokenWithLocation {
                token: Token::Identifier("testvar".to_string()),
                line: 1,
                column: 11,
            },
            TokenWithLocation {
                token: Token::Colon,
                line: 1,
                column: 12,
            },
            TokenWithLocation {
                token: Token::Identifier("bool".to_string()),
                line: 1,
                column: 17,
            },
            TokenWithLocation {
                token: Token::Assign,
                line: 1,
                column: 19,
            },
            TokenWithLocation {
                token: Token::Identifier("testvar".to_string()),
                line: 1,
                column: 27,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 28,
            },
            TokenWithLocation {
                token: Token::EOF,
                line: 1,
                column: 29,
            },
        ];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn test_boolean_variable() {
        let input = r#"var testvar: bool = true; var testvar: bool = false;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            TokenWithLocation {
                token: Token::Identifier("var".to_string()),
                line: 1,
                column: 3,
            },
            TokenWithLocation {
                token: Token::Identifier("testvar".to_string()),
                line: 1,
                column: 11,
            },
            TokenWithLocation {
                token: Token::Colon,
                line: 1,
                column: 12,
            },
            TokenWithLocation {
                token: Token::Identifier("bool".to_string()),
                line: 1,
                column: 17,
            },
            TokenWithLocation {
                token: Token::Assign,
                line: 1,
                column: 19,
            },
            TokenWithLocation {
                token: Token::BooleanLiteral(true),
                line: 1,
                column: 24,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 25,
            },
            TokenWithLocation {
                token: Token::Identifier("var".to_string()),
                line: 1,
                column: 29,
            },
            TokenWithLocation {
                token: Token::Identifier("testvar".to_string()),
                line: 1,
                column: 37,
            },
            TokenWithLocation {
                token: Token::Colon,
                line: 1,
                column: 38,
            },
            TokenWithLocation {
                token: Token::Identifier("bool".to_string()),
                line: 1,
                column: 43,
            },
            TokenWithLocation {
                token: Token::Assign,
                line: 1,
                column: 45,
            },
            TokenWithLocation {
                token: Token::BooleanLiteral(false),
                line: 1,
                column: 51,
            },
            TokenWithLocation {
                token: Token::Semicolon,
                line: 1,
                column: 52,
            },
            TokenWithLocation {
                token: Token::EOF,
                line: 1,
                column: 53,
            },
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
