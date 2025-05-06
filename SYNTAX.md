# Perry Syntax

## End goal

### Calculate factorial
func factorial(n: int): int {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

var result: int = factorial(5);
print(result);


### FizzBuzz up to 15
func fizzbuzz(limit: int): void {
    var i: int = 1;
    while (i <= limit) {
        if (i % 15 == 0) {
            print("FizzBuzz");
        } else if (i % 3 == 0) {
            print("Fizz");
        } else if (i % 5 == 0) {
            print("Buzz");
        } else {
            print(i); // implicitly converts to string
        }
        i = i + 1;
    }
}

fizzbuzz(15);


## To implement
- Keywords
    - var
    - func
    - if
    - else
    - while
    - return
    - print

- Identifiers
- Literals
- Arithemtic Operators
    - +
    - -
    - *
    - /
    - %

- Delimiters
    - ;
    - {
    - }
    - ( )
    - ,

- Boolean Operators
    - ==
    - !=
    - >
    - >=
    - <
    - <=
    - &&
    - ||

