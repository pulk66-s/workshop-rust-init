# Rust Cheatsheet 🦀

A quick reference guide for Rust beginners.

## Table of Contents
- [Basics](#basics)
- [Variables](#variables)
- [Data Types](#data-types)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Ownership](#ownership)
- [References & Borrowing](#references--borrowing)
- [Structs](#structs)
- [Enums](#enums)
- [Pattern Matching](#pattern-matching)
- [Error Handling](#error-handling)
- [Collections](#collections)
- [Traits](#traits)

## Basics

```rust
// Comments
// Single line comment
/* Multi-line comment */

// Printing
println!("Hello, world!");           // Print with newline
print!("No newline");                // Print without newline
println!("Value: {}", 42);           // Print with formatting
println!("x={}, y={}", 10, 20);      // Multiple values
```

## Variables

```rust
// Immutable by default
let x = 5;
// x = 6;  // ERROR: cannot assign twice to immutable variable

// Mutable variables
let mut y = 5;
y = 6;  // OK

// Constants (always immutable, type must be annotated)
const MAX_POINTS: u32 = 100_000;

// Shadowing (can change type)
let spaces = "   ";
let spaces = spaces.len();  // Now spaces is a number
```

## Data Types

```rust
// Integers
let a: i32 = 42;          // Signed 32-bit
let b: u32 = 42;          // Unsigned 32-bit
// i8, i16, i32, i64, i128, isize
// u8, u16, u32, u64, u128, usize

// Floating point
let x: f64 = 3.14;        // 64-bit float
let y: f32 = 3.14;        // 32-bit float

// Boolean
let is_true: bool = true;
let is_false: bool = false;

// Character (4 bytes, Unicode)
let c: char = 'z';
let emoji: char = '😊';

// String types
let s1: &str = "string slice";      // String slice (immutable)
let s2: String = String::from("owned string");  // Owned String

// Tuples
let tuple: (i32, f64, char) = (500, 6.4, 'x');
let (x, y, z) = tuple;              // Destructuring
let first = tuple.0;                // Access by index

// Arrays (fixed size)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let zeros = [0; 5];                 // [0, 0, 0, 0, 0]
```

## Functions

```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// Function with parameters
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon = return value
}

// Explicit return
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

// Multiple return values (tuple)
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}
```

## Control Flow

```rust
// If/Else
let number = 7;
if number < 5 {
    println!("Less than 5");
} else if number < 10 {
    println!("Between 5 and 10");
} else {
    println!("10 or greater");
}

// If as expression
let result = if number > 5 { "big" } else { "small" };

// Loop (infinite)
loop {
    println!("Forever!");
    break;  // Exit the loop
}

// Loop with return value
let result = loop {
    break 42;  // Returns 42
};

// While loop
let mut n = 0;
while n < 5 {
    n += 1;
}

// For loop
for i in 0..5 {          // 0 to 4
    println!("{}", i);
}

for i in 0..=5 {         // 0 to 5 (inclusive)
    println!("{}", i);
}

let arr = [10, 20, 30];
for element in arr.iter() {
    println!("{}", element);
}

// Match (like switch, but exhaustive)
let number = 7;
match number {
    1 => println!("One"),
    2 | 3 => println!("Two or three"),
    4..=6 => println!("Four through six"),
    _ => println!("Something else"),  // Default case
}
```

## Ownership

Rust's core feature! Three rules:
1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

```rust
// Ownership transfer (move)
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2
// println!("{}", s1);  // ERROR: s1 no longer valid

// Clone (deep copy)
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // Both valid

// Copy trait (for stack data)
let x = 5;
let y = x;  // x is copied, both valid
println!("{} {}", x, y);
```

## References & Borrowing

```rust
// Immutable reference (borrow)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // Borrow s1
println!("{} has length {}", s1, len);  // s1 still valid

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the data

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}

// Rules:
// 1. You can have ONE mutable reference OR
// 2. Any number of immutable references
// 3. References must always be valid (no dangling references)
```

## Structs

```rust
// Define a struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Create an instance
let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("user123"),
    active: true,
    sign_in_count: 1,
};

// Access fields
println!("{}", user1.email);

// Mutable instance
let mut user2 = User {
    email: String::from("test2@example.com"),
    username: String::from("user456"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("new@example.com");

// Struct update syntax
let user3 = User {
    email: String::from("test3@example.com"),
    ..user1  // Copy remaining fields from user1
};

// Tuple structs
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);

// Methods
impl User {
    fn full_info(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
    
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 0,
        }
    }
}

let user = User::new(String::from("a@b.com"), String::from("alice"));
println!("{}", user.full_info());
```

## Enums

```rust
// Basic enum
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("Hello"));

// Methods on enums
impl Message {
    fn call(&self) {
        // Method body
    }
}

// Option enum (built-in)
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;
```

## Pattern Matching

```rust
// Match with enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Match with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// If let (for single pattern)
let some_value = Some(3);
if let Some(3) = some_value {
    println!("three");
}
```

## Error Handling

```rust
// Option<T> - for values that might not exist
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

match divide(10.0, 2.0) {
    Some(result) => println!("Result: {}", result),
    None => println!("Cannot divide by zero"),
}

// Result<T, E> - for operations that can fail
use std::fs::File;
use std::io::ErrorKind;

fn open_file() -> Result<File, std::io::Error> {
    File::open("hello.txt")
}

// Match on Result
match open_file() {
    Ok(file) => println!("File opened"),
    Err(error) => println!("Error: {:?}", error),
}

// ? operator (propagate errors)
fn read_username() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("user.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// unwrap and expect (will panic on error)
let f = File::open("file.txt").unwrap();
let f = File::open("file.txt").expect("Failed to open file");
```

## Collections

```rust
// Vec<T> - growable array
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v = vec![1, 2, 3];  // Macro shorthand

// Access elements
let third = &v[2];
let third = v.get(2);  // Returns Option<&T>

// Iterate
for i in &v {
    println!("{}", i);
}

for i in &mut v {
    *i += 1;  // Dereference and modify
}

// String - growable text
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 is moved

let s = format!("{}-{}", "tic", "tac");

// HashMap<K, V>
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Access
let score = scores.get("Blue");  // Returns Option<&V>

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Update
scores.insert(String::from("Blue"), 25);  // Overwrite

// Insert if not exists
scores.entry(String::from("Blue")).or_insert(50);
```

## Traits

```rust
// Define a trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement trait for a type
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// Use trait as parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Common traits
// Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord
```

## Quick Tips

- Use `cargo check` to quickly check if code compiles
- Use `cargo clippy` for helpful linting suggestions
- Use `cargo fmt` to format your code
- Read compiler errors carefully - they're very helpful!
- When stuck with ownership, try using references first
- `println!("{:?}", value)` for debug printing (requires Debug trait)

Happy Rust coding! 🦀
