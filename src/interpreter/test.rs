#![cfg(test)]

mod tests {
    use crate::{interpreter::analyzer::Analyzer, parser::Parser, tokenizer::Lexer};

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
}
