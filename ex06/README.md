# Exercise 06: Structs

## Learning Objectives
- Define structs
- Create struct instances
- Implement methods on structs
- Use associated functions (constructors)
- Understand tuple structs

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define custom structs
   - Create and use struct instances
   - Implement methods
   - Create constructor functions

## Tasks

Create the following:
- A `Rectangle` struct with width and height
- Methods to calculate area and perimeter
- A `new` associated function (constructor)
- A `User` struct with multiple fields
- A tuple struct for a `Point`

## Hints

- Use `struct` keyword to define structs
- Use `impl` block to add methods
- `&self` for methods that borrow, `&mut self` for methods that modify
- No `&self` for associated functions (constructors)

## Running the Exercise

```bash
cd ex06
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html)
