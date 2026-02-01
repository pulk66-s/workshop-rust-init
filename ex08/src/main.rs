// TODO: Implement a function that divides two numbers
// Return Result<f64, String> where the error is "Division by zero"
// fn safe_divide(a: f64, b: f64) -> Result<f64, String>


// TODO: Implement a function that parses a string to i32
// Use the standard library's parse method
// fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError>


// TODO: Implement a function that uses the ? operator
// This function should parse a string and add 10 to it
// fn parse_and_add_ten(s: &str) -> Result<i32, std::num::ParseIntError>


// TODO: Implement a function that gets an element from a vector
// Return Result<&i32, String> with an appropriate error message
// fn get_element(vec: &Vec<i32>, index: usize) -> Result<&i32, String>


// TODO: Implement a function that chains multiple Results
// Parse two strings, multiply them, and return the result
// fn multiply_strings(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError>


fn main() {
    println!("=== Safe Division ===");
    // TODO: Test safe_divide with valid and invalid inputs
    
    
    println!("\n=== Parsing Numbers ===");
    // TODO: Test parse_number with valid and invalid strings
    
    
    println!("\n=== Using ? Operator ===");
    // TODO: Test parse_and_add_ten
    
    
    println!("\n=== Vector Access ===");
    // TODO: Test get_element with valid and invalid indices
    
    
    println!("\n=== Chaining Results ===");
    // TODO: Test multiply_strings
    
}
