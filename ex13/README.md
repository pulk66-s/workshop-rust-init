# Exercise 13: Traits - Interfaces in Rust

## Learning Objectives
- Understand traits as Rust's interface mechanism
- Define and implement traits for custom types
- Use trait bounds for generic programming
- Implement default trait methods
- Work with standard library traits (Debug, Clone, Display)
- Understand trait inheritance

## Background

**Traits** are Rust's way of defining shared behavior. They are similar to interfaces in other languages like Java or TypeScript. A trait defines a set of methods that types can implement.

Key concepts:
- Traits define behavior that can be shared across types
- Any type can implement a trait
- Traits enable polymorphism and generic programming
- Traits can have default implementations

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define custom traits
   - Implement traits for different types
   - Use trait bounds in generic functions
   - Implement standard library traits
   - Create trait inheritance hierarchies

## Tasks

### Part 1: Drawable Trait
Define a `Drawable` trait with:
- `draw(&self)` - Required method to draw the shape
- `describe(&self) -> String` - Default implementation that returns "A drawable shape"

Implement for:
- `Circle` struct (with radius field)
- `Square` struct (with side field)

### Part 2: Comparable Trait
Define a `Comparable` trait with:
- `compare(&self, other: &Self) -> Ordering` - Compare two instances

Implement for:
- `Student` struct (with name and grade fields)
- Compare based on grade

### Part 3: Generic Functions with Trait Bounds
Create generic functions:
- `print_drawable<T: Drawable>(item: &T)` - Print any drawable item
- `find_largest<T: Comparable>(items: &[T]) -> &T` - Find the largest item

### Part 4: Standard Traits
Implement standard library traits for your types:
- `Debug` - For debug printing
- `Clone` - For cloning instances
- `Display` - For user-friendly printing

## Key Concepts

**Trait Definition:**
```rust
trait TraitName {
    fn required_method(&self);
    
    fn default_method(&self) {
        // default implementation
    }
}
```

**Trait Implementation:**
```rust
impl TraitName for TypeName {
    fn required_method(&self) {
        // implementation
    }
}
```

**Trait Bounds:**
```rust
fn function<T: Trait>(item: T) { }
```

## Expected Output

```
Drawing a circle with radius 5
Description: A drawable shape

Drawing a square with side 4
Description: A drawable shape

---

Printing drawable:
Drawing a circle with radius 5

---

Students:
Student { name: "Alice", grade: 85 }
Student { name: "Bob", grade: 92 }
Student { name: "Charlie", grade: 78 }

Best student: Student { name: "Bob", grade: 92 }

---

Circle Display: Circle(radius: 5)
Cloned circle: Circle(radius: 5)
```

## Hints

- Use `#[derive(Debug, Clone)]` to auto-implement Debug and Clone
- Standard library `Ordering` enum has variants: `Less`, `Equal`, `Greater`
- Use `std::fmt::Display` for custom display formatting
- Trait methods can call other trait methods
- Use `where` clauses for complex trait bounds

## Running the Exercise

```bash
cd ex13
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Common Traits](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
