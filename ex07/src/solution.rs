// Direction enum
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Message enum with different variant types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Function that converts Direction to string
fn direction_to_string(dir: Direction) -> &'static str {
    match dir {
        Direction::North => "North",
        Direction::South => "South",
        Direction::East => "East",
        Direction::West => "West",
    }
}

// Function that processes different Message variants
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message received");
        }
        Message::Move { x, y } => {
            println!("Move to position: x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("Write message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

// Function that returns Option<T>
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// Function that works with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Function demonstrating multiple patterns
fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 => "one or two",
        3..=9 => "between three and nine",
        10 => "exactly ten",
        _ => "something else",
    }
}

fn main() {
    println!("=== Direction Enum ===");
    let north = Direction::North;
    let south = Direction::South;
    
    println!("Direction: {}", direction_to_string(north));
    println!("Direction: {:?}", south);
    
    // Match on direction
    let dir = Direction::East;
    match dir {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }
    
    println!("\n=== Message Enum ===");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for msg in messages {
        process_message(msg);
    }
    
    println!("\n=== Option Enum ===");
    match divide(10.0, 2.0) {
        Some(result) => println!("10.0 / 2.0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    match divide(10.0, 0.0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }
    
    // Using plus_one
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five + 1 = {:?}", six);
    println!("none + 1 = {:?}", none);
    
    println!("\n=== If Let ===");
    let some_value = Some(7);
    
    // Instead of:
    // match some_value {
    //     Some(7) => println!("Got 7!"),
    //     _ => (),
    // }
    
    // Use if let:
    if let Some(7) = some_value {
        println!("Got 7!");
    }
    
    if let Some(x) = some_value {
        println!("Got value: {}", x);
    }
    
    println!("\n=== Pattern Matching Examples ===");
    for n in 0..=12 {
        println!("{}: {}", n, describe_number(n));
    }
}
