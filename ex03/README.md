# Exercise 03: Functions

## Learning Objectives
- Define and call functions
- Work with function parameters
- Return values from functions
- Understand expressions vs statements

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Create functions with parameters
   - Return values from functions
   - Use multiple parameters
   - Return multiple values using tuples

## Tasks

Implement the following functions:
- `greet(name: &str)` - prints a greeting
- `add(a: i32, b: i32) -> i32` - returns the sum
- `is_even(n: i32) -> bool` - returns true if n is even
- `calculate(a: i32, b: i32) -> (i32, i32, i32)` - returns sum, difference, and product

## Hints

- Functions use `fn` keyword
- Parameters need type annotations
- Return type is specified with `->`
- Last expression without `;` is the return value
- Use `return` keyword for early returns

## Running the Exercise

```bash
cd ex03
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 3.3](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
