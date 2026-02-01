# Exercise 05: Ownership & Borrowing

## Learning Objectives
- Understand Rust's ownership system
- Learn the three ownership rules
- Practice moving and borrowing values
- Work with references (immutable and mutable)

## The Three Rules of Ownership

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Understand ownership transfer (move)
   - Use references to borrow values
   - Work with mutable references
   - Fix ownership errors

## Tasks

- Observe what happens when ownership is transferred
- Use references to avoid moving values
- Modify values through mutable references
- Understand the borrowing rules

## Key Concepts

**Borrowing Rules:**
- You can have either ONE mutable reference OR any number of immutable references
- References must always be valid

## Hints

- Use `&` to create an immutable reference
- Use `&mut` to create a mutable reference
- The `.clone()` method creates a deep copy
- Stack types (like i32) implement Copy trait

## Running the Exercise

```bash
cd ex05
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
