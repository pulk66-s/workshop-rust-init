use std::f64::consts::PI;

// Define Animal trait
trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

// Define Dog struct
struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Self {
        Dog { name: name.to_string() }
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Define Cat struct
struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Self {
        Cat { name: name.to_string() }
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Define Bird struct
struct Bird {
    name: String,
}

impl Bird {
    fn new(name: &str) -> Self {
        Bird { name: name.to_string() }
    }
}

impl Animal for Bird {
    fn speak(&self) {
        println!("{} says: Chirp!", self.name);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Function that accepts a vector of trait objects
fn all_animals_speak(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.speak();
    }
}

// Define Shape trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Define Circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// Define Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Define Triangle
struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.side1 + self.side2 + self.side3) / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

// Function to calculate total area
fn calculate_total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Define Plugin trait
trait Plugin {
    fn init(&self);
    fn execute(&self, input: &str) -> String;
}

// Define Logger plugin
struct Logger;

impl Plugin for Logger {
    fn init(&self) {
        println!("[Logger] Initializing...");
    }
    
    fn execute(&self, input: &str) -> String {
        format!("LOG: {}", input)
    }
}

// Define Calculator plugin
struct Calculator;

impl Plugin for Calculator {
    fn init(&self) {
        println!("[Calculator] Initializing...");
    }
    
    fn execute(&self, input: &str) -> String {
        format!("CALC: {} = {} chars", input, input.len())
    }
}

// Define Formatter plugin
struct Formatter;

impl Plugin for Formatter {
    fn init(&self) {
        println!("[Formatter] Initializing...");
    }
    
    fn execute(&self, input: &str) -> String {
        format!("FORMAT: *** {} ***", input)
    }
}

fn main() {
    // Part 1 - Create animals and store in a vector of trait objects
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Buddy")),
        Box::new(Cat::new("Whiskers")),
        Box::new(Bird::new("Tweety")),
    ];
    
    println!("All animals speak:");
    all_animals_speak(&animals);
    
    println!("\n---\n");
    
    // Part 2 - Create shapes and calculate total area
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Triangle { side1: 3.0, side2: 4.0, side3: 5.0 }),
    ];
    
    println!("Calculating total area:");
    println!("Circle area: {:.2}", shapes[0].area());
    println!("Rectangle area: {:.2}", shapes[1].area());
    println!("Triangle area: {:.2}", shapes[2].area());
    let total = calculate_total_area(&shapes);
    println!("Total area: {:.2}", total);
    
    println!("\n---\n");
    
    // Part 3 - Create plugins and execute them
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(Logger),
        Box::new(Calculator),
        Box::new(Formatter),
    ];
    
    println!("Plugin system:");
    for plugin in &plugins {
        plugin.init();
    }
    
    println!();
    for plugin in &plugins {
        let result = plugin.execute("Hello World");
        println!("Executing plugin: {}", result);
    }
}
