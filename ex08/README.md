# Exercise 08: Error Handling

## Learning Objectives
- Understand Result<T, E> type
- Use the ? operator for error propagation
- Handle errors with match and if let
- Practice with unwrap, expect, and proper error handling
- Convert between error types

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Work with Result<T, E>
   - Use the ? operator
   - Handle errors gracefully
   - Create custom error types

## Tasks

Implement the following:
- Functions that return Result
- Error propagation with ?
- Parse strings to numbers with error handling
- File operations with proper error handling

## Key Concepts

**Result<T, E>** has two variants:
- `Ok(T)` - success with value T
- `Err(E)` - failure with error E

**The ? operator:**
- Returns early with error if Result is Err
- Unwraps Ok value if Result is Ok

## Hints

- Use `Result<T, E>` for operations that can fail
- The `?` operator can only be used in functions that return Result or Option
- `unwrap()` panics on error (use for prototyping)
- `expect()` panics with a custom message
- `match` or `if let` for proper error handling

## Running the Exercise

```bash
cd ex08
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
