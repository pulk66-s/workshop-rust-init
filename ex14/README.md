# Exercise 14: Trait Objects & Dynamic Dispatch - Polymorphism in Rust

## Learning Objectives
- Understand trait objects and dynamic dispatch
- Use `Box<dyn Trait>` for runtime polymorphism
- Learn the difference between static and dynamic dispatch
- Work with heterogeneous collections
- Understand object safety requirements
- Practice polymorphic design patterns

## Background

**Trait Objects** enable runtime polymorphism in Rust. While generics use **static dispatch** (compile-time), trait objects use **dynamic dispatch** (runtime).

Key concepts:
- `dyn Trait` - Creates a trait object (dynamically sized type)
- `Box<dyn Trait>` - Heap-allocated trait object
- `&dyn Trait` - Reference to a trait object
- Allows storing different types that implement the same trait
- Enables true polymorphism

**Static vs Dynamic Dispatch:**
- **Static**: `fn process<T: Trait>(x: T)` - Compiler generates code for each type
- **Dynamic**: `fn process(x: &dyn Trait)` - Runtime looks up which method to call

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Define traits suitable for trait objects
   - Create heterogeneous collections with trait objects
   - Implement polymorphic behavior
   - Understand object safety

## Tasks

### Part 1: Animal Trait
Define an `Animal` trait with:
- `speak(&self)` - Make the animal sound
- `name(&self) -> &str` - Get the animal's name

Create structs:
- `Dog` with name field
- `Cat` with name field
- `Bird` with name field

Each implements `Animal` with different `speak()` behavior.

### Part 2: Heterogeneous Collection
Create a function that:
- Takes a `Vec<Box<dyn Animal>>`
- Iterates and calls `speak()` on each animal
- Demonstrates polymorphism

### Part 3: Shape Hierarchy
Create a `Shape` trait with:
- `area(&self) -> f64`
- `perimeter(&self) -> f64`

Implement for:
- `Circle` (radius)
- `Rectangle` (width, height)
- `Triangle` (side1, side2, side3)

Create a function that calculates total area of different shapes.

### Part 4: Plugin System Simulation
Create a `Plugin` trait with:
- `init(&self)`
- `execute(&self, input: &str) -> String`

Implement for different plugin types and demonstrate a plugin registry.

## Key Concepts

**Trait Object Syntax:**
```rust
// Box (owned)
let animal: Box<dyn Animal> = Box::new(Dog::new("Buddy"));

// Reference
fn process(animal: &dyn Animal) { }
```

**Object Safety:**
A trait is object-safe if:
- No generic methods
- No methods returning `Self`
- No associated constants

## Expected Output

```
All animals speak:
Buddy says: Woof!
Whiskers says: Meow!
Tweety says: Chirp!

---

Calculating total area:
Circle area: 78.54
Rectangle area: 24.00
Triangle area: 6.00
Total area: 108.54

---

Plugin system:
[Logger] Initializing...
[Calculator] Initializing...
[Formatter] Initializing...

Executing Logger: LOG: Hello World
Executing Calculator: CALC: Hello World = 11 chars
Executing Formatter: FORMAT: *** Hello World ***
```

## Hints

- Use `Box::new(value)` to create a boxed trait object
- Use `vec![]` to create vectors of trait objects
- For circles: area = π × r², perimeter = 2 × π × r
- For rectangles: area = w × h, perimeter = 2(w + h)
- For triangles (Heron's formula): s = (a+b+c)/2, area = √(s(s-a)(s-b)(s-c))
- Use `std::f64::consts::PI` for π

## Running the Exercise

```bash
cd ex14
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 17](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Trait Objects](https://doc.rust-lang.org/reference/types/trait-object.html)
- [Object Safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
