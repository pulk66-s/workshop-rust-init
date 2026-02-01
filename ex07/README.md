# Exercise 07: Enums & Pattern Matching

## Learning Objectives
- Define and use enums
- Work with enum variants that hold data
- Master pattern matching with match
- Use Option and Result enums
- Practice if let syntax

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define custom enums
   - Create enums with data
   - Use match for exhaustive pattern matching
   - Work with Option<T>
   - Use if let for specific patterns

## Tasks

Create the following:
- A `Direction` enum with four variants
- A `Message` enum with different data types
- Functions that use pattern matching
- Practice with Option<T>

## Hints

- Enums can have variants with different data types
- `match` must be exhaustive (cover all cases)
- Use `_` as a catch-all pattern
- `Option<T>` has two variants: `Some(T)` and `None`
- `if let` is useful when you care about one pattern

## Running the Exercise

```bash
cd ex07
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html)
