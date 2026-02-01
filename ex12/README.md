# Exercise 12: Encapsulation and Privacy

## Learning Objectives
- Understand Rust's privacy system
- Practice data hiding with private fields
- Implement getters and setters
- Learn about the `pub` keyword
- Control access to struct internals
- Validate data through controlled interfaces

## Background

**Encapsulation** is one of the fundamental principles of OOP. In Rust, encapsulation is achieved through the module system and privacy rules:

- By default, everything is **private**
- Use `pub` keyword to make items public
- Private fields can only be accessed within the same module
- This allows you to enforce invariants and validate data

## Instructions

1. Open `src/main.rs`
2. Complete the TODO sections to:
   - Create a `BankAccount` struct with private fields
   - Implement getters for read-only access
   - Implement methods that validate operations
   - Create a `Temperature` struct with validation
   - Practice proper encapsulation patterns

## Tasks

### Part 1: BankAccount
Create a `BankAccount` struct with private fields:
- `balance: f64` (private)
- `account_number: String` (private)

Implement methods:
- `new(account_number: String) -> Self` - Create account with 0 balance
- `balance(&self) -> f64` - Getter for balance
- `account_number(&self) -> &str` - Getter for account number
- `deposit(&mut self, amount: f64) -> Result<(), String>` - Add money (must be positive)
- `withdraw(&mut self, amount: f64) -> Result<(), String>` - Remove money (check sufficient funds)

### Part 2: Temperature
Create a `Temperature` struct that stores Celsius with validation:
- `celsius: f64` (private) - Must be >= -273.15 (absolute zero)

Implement methods:
- `from_celsius(celsius: f64) -> Result<Self, String>` - Constructor with validation
- `from_fahrenheit(fahrenheit: f64) -> Result<Self, String>` - Convert and create
- `celsius(&self) -> f64` - Get temperature in Celsius
- `fahrenheit(&self) -> f64` - Get temperature in Fahrenheit
- `kelvin(&self) -> f64` - Get temperature in Kelvin

## Key Concepts

**Privacy Rules:**
- Struct fields are private by default
- Methods can be public even if fields are private
- This creates a "controlled interface" to the data

**Data Validation:**
- Constructors can reject invalid data
- Use `Result<T, E>` to handle validation errors
- Private fields prevent direct invalid assignments

## Expected Output

```
Account: 1234567890
Initial balance: $0.00

Deposited $100.50
Balance: $100.50

Withdrew $30.25
Balance: $70.25

Error: Insufficient funds
Balance: $70.25

---

Temperature: 25.00°C
In Fahrenheit: 77.00°F
In Kelvin: 298.15K

Created from Fahrenheit: 0.00°C
Error: Temperature cannot be below absolute zero (-273.15°C)
```

## Hints

- Use `Result<T, E>` for operations that can fail
- Return `Ok(())` for successful operations that don't return a value
- Return `Err(String::from("error message"))` for errors
- Formula: Fahrenheit = Celsius × 9/5 + 32
- Formula: Kelvin = Celsius + 273.15
- Absolute zero: -273.15°C

## Running the Exercise

```bash
cd ex12
cargo run
```

## Further Reading

- [The Rust Programming Language - Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Visibility](https://doc.rust-lang.org/rust-by-example/mod/visibility.html)
- [Error Handling with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
