// TODO: Define a Direction enum with variants: North, South, East, West


// TODO: Define a Message enum with these variants:
// - Quit (no data)
// - Move { x: i32, y: i32 } (struct-like)
// - Write(String) (tuple-like)
// - ChangeColor(i32, i32, i32) (tuple-like with RGB)


// TODO: Implement a function that takes a Direction and returns a string
// fn direction_to_string(dir: Direction) -> &'static str


// TODO: Implement a function that processes a Message
// fn process_message(msg: Message)


// TODO: Implement a function that uses Option<T>
// fn divide(a: f64, b: f64) -> Option<f64>
// Returns Some(result) if b != 0, None otherwise


// TODO: Implement a function that adds one to an Option<i32>
// fn plus_one(x: Option<i32>) -> Option<i32>


fn main() {
    println!("=== Direction Enum ===");
    // TODO: Create Direction instances and print them
    
    
    println!("\n=== Message Enum ===");
    // TODO: Create different Message variants and process them
    
    
    println!("\n=== Option Enum ===");
    // TODO: Test the divide function with different inputs
    
    
    // TODO: Test plus_one function
    
    
    println!("\n=== If Let ===");
    // TODO: Use if let with Option
    
}
