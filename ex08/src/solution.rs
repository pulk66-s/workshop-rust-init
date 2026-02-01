// Safe division that returns Result
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Parse string to number
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

// Parse and add ten using ? operator
fn parse_and_add_ten(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num = s.parse::<i32>()?;  // ? returns early if Err
    Ok(num + 10)
}

// Get element from vector with bounds checking
fn get_element(vec: &Vec<i32>, index: usize) -> Result<&i32, String> {
    if index < vec.len() {
        Ok(&vec[index])
    } else {
        Err(format!("Index {} out of bounds (len: {})", index, vec.len()))
    }
}

// Chain multiple Results
fn multiply_strings(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError> {
    let num1 = s1.parse::<i32>()?;
    let num2 = s2.parse::<i32>()?;
    Ok(num1 * num2)
}

// Demonstrating different ways to handle errors
fn demonstrate_error_handling() {
    println!("=== Different Error Handling Methods ===");
    
    // 1. Match
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Match: 10.0 / 2.0 = {}", result),
        Err(e) => println!("Match: Error - {}", e),
    }
    
    // 2. if let
    if let Ok(result) = safe_divide(10.0, 2.0) {
        println!("If let: Result = {}", result);
    }
    
    // 3. unwrap_or (provides default value)
    let result = safe_divide(10.0, 0.0).unwrap_or(0.0);
    println!("Unwrap or: {}", result);
    
    // 4. unwrap_or_else (computes default value)
    let result = safe_divide(10.0, 0.0).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        -1.0
    });
    println!("Unwrap or else: {}", result);
}

fn main() {
    println!("=== Safe Division ===");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Parsing Numbers ===");
    match parse_number("42") {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    match parse_number("not a number") {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    println!("\n=== Using ? Operator ===");
    match parse_and_add_ten("32") {
        Ok(result) => println!("32 + 10 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_and_add_ten("invalid") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Vector Access ===");
    let numbers = vec![1, 2, 3, 4, 5];
    
    match get_element(&numbers, 2) {
        Ok(value) => println!("Element at index 2: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    match get_element(&numbers, 10) {
        Ok(value) => println!("Element at index 10: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Chaining Results ===");
    match multiply_strings("5", "10") {
        Ok(result) => println!("5 * 10 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match multiply_strings("5", "not a number") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!();
    demonstrate_error_handling();
}
