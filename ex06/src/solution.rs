// Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods for Rectangle
impl Rectangle {
    // Constructor (associated function)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Calculate perimeter
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // Check if it's a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // Method that modifies the struct
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// User struct
struct User {
    username: String,
    email: String,
    age: u32,
}

impl User {
    // Constructor
    fn new(username: String, email: String, age: u32) -> User {
        User {
            username,
            email,
            age,
        }
    }
    
    // Display user information
    fn display_info(&self) {
        println!("User: {}", self.username);
        println!("Email: {}", self.email);
        println!("Age: {}", self.age);
    }
}

// Tuple struct for a 2D point
struct Point(f64, f64);

impl Point {
    // Calculate distance from origin (0, 0)
    fn distance_from_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    
    // Constructor
    fn new(x: f64, y: f64) -> Point {
        Point(x, y)
    }
}

fn main() {
    // Create and use Rectangle
    let rect = Rectangle::new(10, 20);
    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());
    
    let square = Rectangle::new(5, 5);
    println!("\nSquare: {}x{}", square.width, square.height);
    println!("Is square: {}", square.is_square());
    
    // Mutable rectangle
    let mut resizable = Rectangle::new(3, 4);
    println!("\nBefore scaling: {}x{}", resizable.width, resizable.height);
    resizable.scale(2);
    println!("After scaling: {}x{}", resizable.width, resizable.height);
    
    // Create and use User
    println!("\n--- User Info ---");
    let user = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
        25
    );
    user.display_info();
    
    // Create and use Point
    println!("\n--- Point Info ---");
    let point = Point::new(3.0, 4.0);
    println!("Point: ({}, {})", point.0, point.1);
    println!("Distance from origin: {:.2}", point.distance_from_origin());
}
