use std::cmp::Ordering;
use std::fmt;

// Define a Drawable trait
trait Drawable {
    // Required method - no default implementation
    fn draw(&self);
    
    // Default method - provides a default implementation
    fn describe(&self) -> String {
        String::from("A drawable shape")
    }
}

// Define Circle with Debug and Clone derived
#[derive(Debug, Clone)]
struct Circle {
    radius: u32,
}

// Implement Drawable for Circle
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Implement Display for Circle
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(radius: {})", self.radius)
    }
}

// Define Square with Debug and Clone derived
#[derive(Debug, Clone)]
struct Square {
    side: u32,
}

// Implement Drawable for Square
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

// Define a Comparable trait
trait Comparable {
    fn compare(&self, other: &Self) -> Ordering;
}

// Define Student with Debug and Clone derived
#[derive(Debug, Clone)]
struct Student {
    name: String,
    grade: u32,
}

// Implement Comparable for Student (compare by grade)
impl Comparable for Student {
    fn compare(&self, other: &Self) -> Ordering {
        self.grade.cmp(&other.grade)
    }
}

// Generic function that prints any Drawable
fn print_drawable<T: Drawable>(item: &T) {
    println!("Printing drawable:");
    item.draw();
}

// Generic function to find the largest Comparable item
fn find_largest<T: Comparable>(items: &[T]) -> &T {
    let mut largest = &items[0];
    for item in items.iter() {
        if item.compare(largest) == Ordering::Greater {
            largest = item;
        }
    }
    largest
}

fn main() {
    // Create a Circle and test Drawable
    let circle = Circle { radius: 5 };
    circle.draw();
    println!("Description: {}\n", circle.describe());
    
    // Create a Square and test Drawable
    let square = Square { side: 4 };
    square.draw();
    println!("Description: {}\n", square.describe());
    
    println!("---\n");
    
    // Test generic print_drawable function
    print_drawable(&circle);
    
    println!("\n---\n");
    
    // Create students and test Comparable
    let students = vec![
        Student { name: String::from("Alice"), grade: 85 },
        Student { name: String::from("Bob"), grade: 92 },
        Student { name: String::from("Charlie"), grade: 78 },
    ];
    
    println!("Students:");
    for student in &students {
        println!("{:?}", student);
    }
    
    // Find the best student
    let best = find_largest(&students);
    println!("\nBest student: {:?}", best);
    
    println!("\n---\n");
    
    // Test Display and Clone
    println!("Circle Display: {}", circle);
    let cloned_circle = circle.clone();
    println!("Cloned circle: {}", cloned_circle);
}
