// Define a Rectangle struct with width and height fields
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// Implement methods for Rectangle
impl Rectangle {
    // Constructor (associated function)
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // Method to calculate area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to calculate perimeter
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Method to check if it's a square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Method to scale the rectangle (mutates self)
    pub fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Define a Person struct with name and age
pub struct Person {
    pub name: String,
    pub age: u32,
}

// Implement methods for Person
impl Person {
    // Constructor
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    // Method to introduce the person
    pub fn introduce(&self) {
        println!("Hi, I'm {} and I'm {} years old", self.name, self.age);
    }

    // Method to increment age (birthday)
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }

    // Method to check if person is an adult
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    // Create a Rectangle with width 10 and height 20
    let mut rect = Rectangle::new(10, 20);
    
    // Print the rectangle information
    println!("Rectangle: {}x{}", rect.width, rect.height);
    
    // Calculate and print area
    println!("Area: {}", rect.area());
    
    // Calculate and print perimeter
    println!("Perimeter: {}", rect.perimeter());
    
    // Check and print if it's a square
    println!("Is square: {}", rect.is_square());
    
    // Scale the rectangle by 2
    println!("\nAfter scaling by 2:");
    rect.scale(2);
    println!("Rectangle: {}x{}", rect.width, rect.height);
    
    println!("\n---\n");
    
    // Create a Person named "Alice" with age 25
    let mut person = Person::new(String::from("Alice"), 25);
    
    // Make the person introduce themselves
    person.introduce();
    
    // Check if they are an adult
    println!("Is adult: {}", person.is_adult());
    
    // Give them a birthday
    println!("Happy birthday!");
    person.have_birthday();
    
    // Introduce again to show age changed
    person.introduce();
}
