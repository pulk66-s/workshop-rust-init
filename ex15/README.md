# Exercise 15: Advanced OOP Patterns - Builder, Composition & More

## Learning Objectives
- Implement the Builder pattern for complex object construction
- Understand composition over inheritance
- Work with associated types in traits
- Implement the Newtype pattern
- Practice the Strategy pattern
- Create type-safe builders with fluent APIs

## Background

Rust doesn't have traditional inheritance, but it has powerful alternatives:
- **Composition**: Structs containing other structs
- **Traits**: Shared behavior without inheritance
- **Design Patterns**: Builder, Strategy, Newtype, etc.

These patterns are often more flexible and maintainable than inheritance-based designs.

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Implement the Builder pattern
   - Use composition to build complex types
   - Create associated types in traits
   - Implement the Newtype pattern
   - Apply the Strategy pattern

## Tasks

### Part 1: Builder Pattern
Create a `User` struct with many optional fields:
- `username: String` (required)
- `email: Option<String>`
- `age: Option<u32>`
- `bio: Option<String>`

Create a `UserBuilder` that:
- Uses method chaining
- Validates data
- Returns `Result<User, String>`

### Part 2: Composition
Create an `Engine` struct and a `Car` struct that contains an `Engine`:
- `Engine` has: `horsepower`, `fuel_type`
- `Car` has: `model`, `engine: Engine`, `mileage`

Demonstrate composition by having `Car` methods that delegate to `Engine`.

### Part 3: Associated Types
Create an `Iterator`-like trait with associated types:
- `Container` trait with `Item` associated type
- Implement for custom collection types

### Part 4: Newtype Pattern
Create a `Email` newtype that wraps `String`:
- Validates email format at construction
- Provides type safety

### Part 5: Strategy Pattern
Create different sorting strategies:
- `SortStrategy` trait
- `BubbleSort`, `QuickSort` implementations
- `Sorter` struct that uses any strategy

## Key Concepts

**Builder Pattern:**
```rust
let user = UserBuilder::new("alice")
    .email("alice@example.com")
    .age(25)
    .build()?;
```

**Composition:**
```rust
struct Car {
    engine: Engine,  // Has-a relationship
}
```

**Associated Types:**
```rust
trait Container {
    type Item;
    fn get(&self) -> Self::Item;
}
```

**Newtype:**
```rust
struct Email(String);  // Wraps String with type safety
```

## Expected Output

```
=== Builder Pattern ===
User created: alice (alice@example.com), age 25
User with minimal info: bob

Error creating user: Invalid email format

=== Composition ===
Car: Tesla Model S
Engine: 1020hp, Electric
Starting the car...
Engine started!
Stopping the car...
Engine stopped!

=== Newtype Pattern ===
Valid email: alice@example.com
Error: Invalid email format: not-an-email

=== Strategy Pattern ===
Original: [5, 2, 8, 1, 9]
Bubble sorted: [1, 2, 5, 8, 9]
Quick sorted: [1, 2, 5, 8, 9]
```

## Hints

- Builder methods return `self` for method chaining
- Use `Option<T>` for optional fields
- Validate in the `build()` method
- Composition is implemented by including struct as a field
- Associated types use `type Name = Type;` syntax
- Newtype pattern: `struct NewType(InnerType);`
- Strategy pattern uses trait objects or generics

## Running the Exercise

```bash
cd ex15
cargo run
```

## Further Reading

- [The Rust Programming Language - Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Builder Pattern in Rust](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
- [Newtype Pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
- [Associated Types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
