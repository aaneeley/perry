#![cfg(test)]

use super::*;

mod tests {
    use super::*;

    #[test]
    fn token_simple_print() {
        let input = r#"print("Hello", "World!");"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("print".to_string()).spanned(Span { line: 1, column: 5 }),
            Token::LeftParen.spanned(Span { line: 1, column: 6 }),
            Token::StringLiteral("Hello".to_string()).spanned(Span {
                line: 1,
                column: 13,
            }),
            Token::Comma.spanned(Span {
                line: 1,
                column: 14,
            }),
            Token::StringLiteral("World!".to_string()).spanned(Span {
                line: 1,
                column: 23,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 24,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 25,
            }),
            Token::EOF.spanned(Span {
                line: 1,
                column: 26,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_single_line_comment_skip() {
        let input = r#"print("Hello", "World!"); // comment 123 ([]}};; +- comment
        // comment 123 ([]}};; +- comment"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("print".to_string()).spanned(Span { line: 1, column: 5 }),
            Token::LeftParen.spanned(Span { line: 1, column: 6 }),
            Token::StringLiteral("Hello".to_string()).spanned(Span {
                line: 1,
                column: 13,
            }),
            Token::Comma.spanned(Span {
                line: 1,
                column: 14,
            }),
            Token::StringLiteral("World!".to_string()).spanned(Span {
                line: 1,
                column: 23,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 24,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 25,
            }),
            Token::EOF.spanned(Span {
                line: 2,
                column: 42,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_multi_line_comment_skip() {
        let input = r#"print("Hello",/*Comment*/ "World!"); /* comment 123 ([]}};; +- comment
         comment 123 ([]}};; +- comment*/"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("print".to_string()).spanned(Span { line: 1, column: 5 }),
            Token::LeftParen.spanned(Span { line: 1, column: 6 }),
            Token::StringLiteral("Hello".to_string()).spanned(Span {
                line: 1,
                column: 13,
            }),
            Token::Comma.spanned(Span {
                line: 1,
                column: 14,
            }),
            Token::StringLiteral("World!".to_string()).spanned(Span {
                line: 1,
                column: 34,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 35,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 36,
            }),
            Token::EOF.spanned(Span {
                line: 2,
                column: 42,
            }),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_artihmetic_print() {
        let input = r#"println((1 + 2) * 3 / 4);"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("println".to_string()).spanned(Span { line: 1, column: 7 }),
            Token::LeftParen.spanned(Span { line: 1, column: 8 }),
            Token::LeftParen.spanned(Span { line: 1, column: 9 }),
            Token::NumericLiteral(1).spanned(Span {
                line: 1,
                column: 10,
            }),
            Token::BinaryOperator(BinaryOperator::Add).spanned(Span {
                line: 1,
                column: 12,
            }),
            Token::NumericLiteral(2).spanned(Span {
                line: 1,
                column: 14,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 15,
            }),
            Token::BinaryOperator(BinaryOperator::Multiply).spanned(Span {
                line: 1,
                column: 17,
            }),
            Token::NumericLiteral(3).spanned(Span {
                line: 1,
                column: 19,
            }),
            Token::BinaryOperator(BinaryOperator::Divide).spanned(Span {
                line: 1,
                column: 21,
            }),
            Token::NumericLiteral(4).spanned(Span {
                line: 1,
                column: 23,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 24,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 25,
            }),
            Token::EOF.spanned(Span {
                line: 1,
                column: 26,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_arithmetic_bool_variable() {
        let input = r#"var testvar: bool = (5 >= (4 - 3));"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("var".to_string()).spanned(Span { line: 1, column: 3 }),
            Token::Identifier("testvar".to_string()).spanned(Span {
                line: 1,
                column: 11,
            }),
            Token::Colon.spanned(Span {
                line: 1,
                column: 12,
            }),
            Token::Identifier("bool".to_string()).spanned(Span {
                line: 1,
                column: 17,
            }),
            Token::Assign.spanned(Span {
                line: 1,
                column: 19,
            }),
            Token::LeftParen.spanned(Span {
                line: 1,
                column: 21,
            }),
            Token::NumericLiteral(5).spanned(Span {
                line: 1,
                column: 22,
            }),
            Token::BinaryOperator(BinaryOperator::GreaterThanOrEqual).spanned(Span {
                line: 1,
                column: 25,
            }),
            Token::LeftParen.spanned(Span {
                line: 1,
                column: 27,
            }),
            Token::NumericLiteral(4).spanned(Span {
                line: 1,
                column: 28,
            }),
            Token::BinaryOperator(BinaryOperator::Subtract).spanned(Span {
                line: 1,
                column: 30,
            }),
            Token::NumericLiteral(3).spanned(Span {
                line: 1,
                column: 32,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 33,
            }),
            Token::RightParen.spanned(Span {
                line: 1,
                column: 34,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 35,
            }),
            Token::EOF.spanned(Span {
                line: 1,
                column: 36,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_variable_reference() {
        let input = r#"var testvar: bool = testvar;"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("var".to_string()).spanned(Span { line: 1, column: 3 }),
            Token::Identifier("testvar".to_string()).spanned(Span {
                line: 1,
                column: 11,
            }),
            Token::Colon.spanned(Span {
                line: 1,
                column: 12,
            }),
            Token::Identifier("bool".to_string()).spanned(Span {
                line: 1,
                column: 17,
            }),
            Token::Assign.spanned(Span {
                line: 1,
                column: 19,
            }),
            Token::Identifier("testvar".to_string()).spanned(Span {
                line: 1,
                column: 27,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 28,
            }),
            Token::EOF.spanned(Span {
                line: 1,
                column: 29,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn token_boolean_variable() {
        let input = r#"var testvar: bool = true; var testvar: bool = false;"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("var".to_string()).spanned(Span { line: 1, column: 3 }),
            Token::Identifier("testvar".to_string()).spanned(Span {
                line: 1,
                column: 11,
            }),
            Token::Colon.spanned(Span {
                line: 1,
                column: 12,
            }),
            Token::Identifier("bool".to_string()).spanned(Span {
                line: 1,
                column: 17,
            }),
            Token::Assign.spanned(Span {
                line: 1,
                column: 19,
            }),
            Token::BooleanLiteral(true).spanned(Span {
                line: 1,
                column: 24,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 25,
            }),
            Token::Identifier("var".to_string()).spanned(Span {
                line: 1,
                column: 29,
            }),
            Token::Identifier("testvar".to_string()).spanned(Span {
                line: 1,
                column: 37,
            }),
            Token::Colon.spanned(Span {
                line: 1,
                column: 38,
            }),
            Token::Identifier("bool".to_string()).spanned(Span {
                line: 1,
                column: 43,
            }),
            Token::Assign.spanned(Span {
                line: 1,
                column: 45,
            }),
            Token::BooleanLiteral(false).spanned(Span {
                line: 1,
                column: 51,
            }),
            Token::Semicolon.spanned(Span {
                line: 1,
                column: 52,
            }),
            Token::EOF.spanned(Span {
                line: 1,
                column: 53,
            }),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn token_all() {
        let input = r#"ab1"string"123 false + - * / % != == > < >= <=
        ! , = () {} ; : ="#
            .to_string();
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let expected = vec![
            Token::Identifier("ab1".to_string()).spanned(Span { line: 1, column: 3 }),
            Token::StringLiteral("string".to_string()).spanned(Span {
                line: 1,
                column: 11,
            }),
            Token::NumericLiteral(123).spanned(Span {
                line: 1,
                column: 14,
            }),
            Token::BooleanLiteral(false).spanned(Span {
                line: 1,
                column: 20,
            }),
            Token::BinaryOperator(BinaryOperator::Add).spanned(Span {
                line: 1,
                column: 22,
            }),
            Token::BinaryOperator(BinaryOperator::Subtract).spanned(Span {
                line: 1,
                column: 24,
            }),
            Token::BinaryOperator(BinaryOperator::Multiply).spanned(Span {
                line: 1,
                column: 26,
            }),
            Token::BinaryOperator(BinaryOperator::Divide).spanned(Span {
                line: 1,
                column: 28,
            }),
            Token::BinaryOperator(BinaryOperator::Modulo).spanned(Span {
                line: 1,
                column: 30,
            }),
            Token::BinaryOperator(BinaryOperator::NotEqual).spanned(Span {
                line: 1,
                column: 33,
            }),
            Token::BinaryOperator(BinaryOperator::Equal).spanned(Span {
                line: 1,
                column: 36,
            }),
            Token::BinaryOperator(BinaryOperator::GreaterThan).spanned(Span {
                line: 1,
                column: 38,
            }),
            Token::BinaryOperator(BinaryOperator::LessThan).spanned(Span {
                line: 1,
                column: 40,
            }),
            Token::BinaryOperator(BinaryOperator::GreaterThanOrEqual).spanned(Span {
                line: 1,
                column: 43,
            }),
            Token::BinaryOperator(BinaryOperator::LessThanOrEqual).spanned(Span {
                line: 1,
                column: 46,
            }),
            Token::UnaryOperator(UnaryOperator::Not).spanned(Span { line: 2, column: 9 }),
            Token::Comma.spanned(Span {
                line: 2,
                column: 11,
            }),
            Token::Assign.spanned(Span {
                line: 2,
                column: 13,
            }),
            Token::LeftParen.spanned(Span {
                line: 2,
                column: 15,
            }),
            Token::RightParen.spanned(Span {
                line: 2,
                column: 16,
            }),
            Token::LeftBrace.spanned(Span {
                line: 2,
                column: 18,
            }),
            Token::RightBrace.spanned(Span {
                line: 2,
                column: 19,
            }),
            Token::Semicolon.spanned(Span {
                line: 2,
                column: 21,
            }),
            Token::Colon.spanned(Span {
                line: 2,
                column: 23,
            }),
            Token::Assign.spanned(Span {
                line: 2,
                column: 25,
            }),
            Token::EOF.spanned(Span {
                line: 2,
                column: 26,
            }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_invalid_symbol() {
        let input = r#"print("Hello", "World!"?"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize();
        assert!(tokens.is_err());
    }
}
