# Exercise 09: Collections

## Learning Objectives
- Work with Vec<T> (vectors)
- Use HashMap<K, V>
- Understand String vs &str
- Iterate over collections
- Perform common collection operations

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Create and manipulate vectors
   - Work with HashMaps
   - Practice String operations
   - Iterate over collections

## Tasks

Implement the following:
- Vector operations (push, pop, access, iterate)
- HashMap operations (insert, get, update)
- String manipulation
- Collection iteration patterns

## Key Concepts

**Vec<T>:**
- Growable array
- Stored on heap
- Can push/pop elements

**HashMap<K, V>:**
- Key-value pairs
- Keys must implement Hash and Eq
- Use entry API for updates

**String:**
- Owned, growable text
- UTF-8 encoded

## Hints

- Create vectors with `Vec::new()` or `vec![]` macro
- Use `.push()` and `.pop()` for vectors
- Import HashMap: `use std::collections::HashMap;`
- Use `.entry()` and `.or_insert()` for HashMap updates
- Iterate with `for item in collection.iter()`

## Running the Exercise

```bash
cd ex09
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
