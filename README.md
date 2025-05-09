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
#### Code:
```go
var x: int = 1;
y = 4;
```
#### Output:
```diff
- SemanticError: cannot assign to undeclared identifier y (at 2:1)
```

### More Examples

Check out the [working_examples](./working_examples/) directory to see more known-working examples.
