# Perry

#### An interpreted programming language built from scratch in Rust.

> [!IMPORTANT]  
> This project is inteded to be educational and showcase my experience with data structures and algorithmic concepts. It is not meant to be used in any serious context.

## Language Features

### Familiar Syntax
```go
// Single line comments

/*
And
multi-line
comments
*/

func fizzbuzz(limit: int): void {
    var i: int = 1;
    while (i <= limit) {
        if (i % 15 == 0) {
            println("FizzBuzz");
        } else if (i % 3 == 0) {
            println("Fizz");
        } else if (i % 5 == 0) {
            println("Buzz");
        } else {
            println(i); 
        }
        i = i + 1;
    }
}

fizzbuzz(15);
```
### Strong Typing
```go
func a(input: bool): int {
    // Semantic analyzer forces you to return an int
}
var b: string = "Hello, World!"; // And assign the correct type
```

### Seamless Recursion
```go
/*
Calculate factorial of 5
*/

func factorial(n: int): int {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

var result: int = factorial(5);
println(result);
```

### Meaningful Error Messages
#### Code:
```go
print("Missing paren";
```
#### Output:
```diff
- SyntaxError: expected RightParen, got Semicolon (at 1:22)
```
---
(Includes semantic analysis pre-execution)
#### Code:
```go
var x: int = 1;
y = 4;
```
#### Output:
```diff
- SemanticError: cannot assign to undeclared identifier y (at 2:1)
```
---
#### Code:
```go
var x: string = 1;
```
#### Output:
```diff
- SemanticError: variable x declared with type string but assigned with type int (at 1:16)
```

### More Examples

Check out the [working_examples](./working_examples/) directory to see more known-working examples.

## More about this project

The goal of this project was to implement as much of the code by hand, meaning I used no external lexer, tokenizer, or parser. Everything is made using basic algorithms and data structures.

The tokenizer/lexer is a simple finite-state-machine. It takes the raw source code input as a string and tokenizes the input for use by the parser.

The parser is an LL(1) recursive descent parser with a little bit of Pratt parsing for binary operations. It contructs an Abstract Syntax Tree (AST) for use in semantic analysis and execution. The semantic analyzer takes the AST and does type checking as well as tracking declarations/references in a symbol table. If any errors are found, they are raised before the interpreter attempts execution.

Finally, the interpreter takes the (now validated) AST and executes it, handling any runtime errors and maintaining symbols in a scope stack.
