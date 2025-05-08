#![cfg(test)]

use crate::{parser::Parser, tokenizer::Lexer};

use super::Analyzer;

mod tests {

    use super::*;

    #[test]
    fn valid_variable_declaration() {
        let input = r#"var testvar: bool = 1 > 4;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        Analyzer::new(&ast).analyze().unwrap();
    }

    #[test]
    fn mismatched_variable_declaration() {
        let input = r#"var testvar: string = 1 + 4;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = Analyzer::new(&ast).analyze();
        assert!(result.is_err());
    }

    #[test]
    fn mismatched_binary_expression_types() {
        let input_cases = vec![
            r#"var testvar: int = 1 + true;"#,
            r#"var testvar: string = "stringliteral" + 4;"#,
            r#"var testvar: bool = true == 4;"#,
        ];
        for input in input_cases {
            let mut lexer = Lexer::new(input.to_string());
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            let result = Analyzer::new(&ast).analyze();
            assert!(result.is_err());
        }
    }

    #[test]
    fn valid_if_chain() {
        let input = r#"
        if (1 > 2) {
        } else if (2 > 1) {
        } else {
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
    }

    #[test]
    fn valid_function() {
        let input = r#"func name(n: int): int {
            return n;
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
    }

    #[test]
    fn function_with_mismatched_return() {
        let input = r#"func name(n: int): int {
            return "string";
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = Analyzer::new(&ast).analyze();
        assert!(result.is_err());
    }

    #[test]
    fn void_function_with_return_val() {
        let input = r#"func name(n: int): void {
            return 1;
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = Analyzer::new(&ast).analyze();
        assert!(result.is_err());
    }

    #[test]
    fn function_with_missing_return() {
        let input = r#"func name(n: int): int {
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = Analyzer::new(&ast).analyze();
        assert!(result.is_err());
    }
}
