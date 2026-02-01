// Function that greets someone
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

// Function that checks if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Function that performs multiple calculations
fn calculate(a: i32, b: i32) -> (i32, i32, i32) {
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    (sum, difference, product)
}

// Bonus: Function with early return
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;  // Early return
    }
    a / b
}

fn main() {
    // Call the greet function
    greet("Alice");
    greet("Bob");
    
    // Call the add function and print result
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Test is_even function
    println!("Is 4 even? {}", is_even(4));
    println!("Is 7 even? {}", is_even(7));
    
    // Call calculate and destructure the result
    let (sum, diff, prod) = calculate(10, 5);
    println!("10 and 5: sum={}, difference={}, product={}", sum, diff, prod);
    
    // Bonus: test divide function
    println!("10.0 / 2.0 = {}", divide(10.0, 2.0));
    println!("10.0 / 0.0 = {}", divide(10.0, 0.0));
}
