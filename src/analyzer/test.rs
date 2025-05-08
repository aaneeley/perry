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
    fn duplicate_variable_declaration() {
        let input = r#"var testvar: int = 1 + 4; var testvar: int = 1 + 4;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = Analyzer::new(&ast).analyze();
        assert!(result.is_err());
    }

    #[test]
    fn undefined_variable_reference() {
        let input = r#"if(a == b) {}"#;
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

    #[test]
    fn return_outside_function() {
        let input = r#"return 1;"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn valid_loop() {
        let input = r#"
        while (true) {
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
    }

    #[test]
    fn invalid_loop() {
        let input = r#"
        while (4) {
        }"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn valid_function_call() {
        let input = r#"func name(n: int): int {
            return n;
        }
        name(1);"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
    }

    #[test]
    fn mismatched_type_function_call() {
        let input = r#"func name(n: int): int {
            return n;
        }
        name("Hello");"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn too_mant_args_function_call() {
        let input = r#"func name(n: int): int {
            return n;
        }
        name(1, 2);"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn too_few_args_function_call() {
        let input = r#"func name(n: int): int {
            return n;
        }
        name();"#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        let result = analyzer.analyze();
        assert!(result.is_err());
    }
}
