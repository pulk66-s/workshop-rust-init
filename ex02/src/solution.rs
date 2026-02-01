fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause an error!
    
    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The value of y is now: {}", y);
    
    // Explicit type annotation
    let age: u32 = 25;
    println!("Age: {}", age);
    
    // Boolean and char
    let is_learning = true;
    let grade: char = 'A';
    println!("Is learning: {}, Grade: {}", is_learning, grade);
    
    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);
    
    // Different numeric types
    let integer: i32 = -42;
    let unsigned: u32 = 42;
    let float: f64 = 3.14159;
    
    println!("Integer: {}", integer);
    println!("Unsigned: {}", unsigned);
    println!("Float: {}", float);
    
    // Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}", 
             sum, difference, product, quotient, remainder);
}
