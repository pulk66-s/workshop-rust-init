# Exercise 10: Traits

## Learning Objectives
- Define traits
- Implement traits for custom types
- Use trait bounds
- Understand common standard library traits
- Practice trait objects

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define custom traits
   - Implement traits for structs
   - Use trait bounds in functions
   - Work with standard library traits

## Tasks

Implement the following:
- A `Summary` trait with a summarize method
- Implement the trait for different types
- A function that accepts any type implementing the trait
- Implement common traits like Display, Debug, Clone

## Key Concepts

**Traits:**
- Similar to interfaces in other languages
- Define shared behavior
- Can have default implementations

**Common traits:**
- `Debug` - for {:?} formatting
- `Display` - for {} formatting
- `Clone` - for .clone()
- `PartialEq` - for == comparison

## Hints

- Use `trait` keyword to define traits
- `impl TraitName for TypeName` to implement
- `where T: TraitName` for trait bounds
- `#[derive(Debug, Clone)]` for auto-implementation
- `&impl Trait` or `&dyn Trait` for trait parameters

## Running the Exercise

```bash
cd ex10
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html)
