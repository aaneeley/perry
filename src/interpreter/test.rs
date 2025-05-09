#![cfg(test)]

use crate::analyzer::Analyzer;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

mod test {
    use super::*;

    #[test]
    // A complex example that tests most of the features of the language:
    // multiple functions, loops, if statements, variable declarations,
    // arithmetic, variable re-assignments, and recursion.
    //
    // Not perfect, but it covert 90% of edge cases without having 10000 tests.
    fn complex_full() {
        let input = r#"func is_prime(n: int): bool {if (n <= 1) {return false;}
        var i: int = 2;while (i < n) {if (n % i == 0) {return false;}i = i + 1;
        }return true;}func factorial(o: int): int {if (o <= 1) {return 1;}
        return o * factorial(o - 1);}func complex(limit: int): void 
        {var i: int = 1;while (i <= limit) {if (i % 2 == 0) {println("Even");}
        else {println("Odd");}if (is_prime(i)) {println("Prime number:");
        println(i);} var fact: int = factorial(i);println("Factorial:");
        println(fact);i = i + 1;}}complex(5);"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
        let mut interpreter = Interpreter::new(&ast);
        interpreter.execute().unwrap();
    }

    #[test]
    fn builtin_print() {
        let input = r#"println("Hello, World!");"#;
        let mut tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut analyzer = Analyzer::new(&ast);
        analyzer.analyze().unwrap();
        let mut interpreter = Interpreter::new(&ast);
        interpreter.execute().unwrap();
    }
}
