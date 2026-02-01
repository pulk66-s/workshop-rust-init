# Exercise 11: Structs and Methods - Basic OOP

## Learning Objectives
- Define structs to encapsulate data
- Implement methods on structs using `impl` blocks
- Understand `self`, `&self`, and `&mut self`
- Create associated functions (like constructors)
- Practice basic object-oriented programming in Rust

## Background

In Rust, Object-Oriented Programming is achieved primarily through **structs** and **traits**. Unlike classes in traditional OOP languages, Rust separates data (structs) from behavior (methods and traits).

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define a `Rectangle` struct with width and height
   - Implement a constructor method (`new`)
   - Implement methods to calculate area and perimeter
   - Implement a method to check if it's a square
   - Define a `Person` struct with private fields
   - Implement methods to interact with Person

## Tasks

### Part 1: Rectangle Struct
Create a `Rectangle` struct with:
- `new(width: u32, height: u32) -> Self` - Associated function (constructor)
- `area(&self) -> u32` - Calculate area
- `perimeter(&self) -> u32` - Calculate perimeter
- `is_square(&self) -> bool` - Check if width equals height
- `scale(&mut self, factor: u32)` - Multiply dimensions by factor

### Part 2: Person Struct
Create a `Person` struct with:
- Fields: name (String), age (u32)
- `new(name: String, age: u32) -> Self` - Constructor
- `introduce(&self)` - Print introduction
- `have_birthday(&mut self)` - Increment age by 1
- `is_adult(&self) -> bool` - Check if age >= 18

## Key Concepts

**Associated Functions vs Methods:**
- Associated functions: `Rectangle::new(10, 20)` - no `self` parameter
- Methods: `rect.area()` - takes `self` in some form

**Self Types:**
- `self` - Takes ownership (consumes the value)
- `&self` - Immutable borrow (read-only)
- `&mut self` - Mutable borrow (can modify)

## Expected Output

```
Rectangle: 10x20
Area: 200
Perimeter: 60
Is square: false

After scaling by 2:
Rectangle: 20x40

---

Hi, I'm Alice and I'm 25 years old
Is adult: true
Happy birthday!
Hi, I'm Alice and I'm 26 years old
```

## Hints

- Use `impl StructName { }` to define methods
- Associated functions don't have `self` parameter
- Methods are called with dot notation: `rect.area()`
- Associated functions are called with `::` syntax: `Rectangle::new(...)`

## Running the Exercise

```bash
cd ex11
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [Rust by Example - Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
