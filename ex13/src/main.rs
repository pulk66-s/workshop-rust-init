use std::cmp::Ordering;
use std::fmt;

// TODO: Define a Drawable trait
// trait Drawable {
//     // Required method - no default implementation
//     fn draw(&self);
//     
//     // Default method - provides a default implementation
//     fn describe(&self) -> String {
//         String::from("A drawable shape")
//     }
// }


// TODO: Define a Circle struct with radius field
// Add #[derive(Debug, Clone)] before the struct
// #[derive(Debug, Clone)]
// struct Circle {
//     radius: u32,
// }


// TODO: Implement Drawable for Circle
// impl Drawable for Circle {
//     fn draw(&self) {
//         println!("Drawing a circle with radius {}", self.radius);
//     }
// }


// TODO: Implement Display for Circle
// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle(radius: {})", self.radius)
//     }
// }


// TODO: Define a Square struct with side field
// Add #[derive(Debug, Clone)]


// TODO: Implement Drawable for Square
// impl Drawable for Square {
//     fn draw(&self) {
//         println!("Drawing a square with side {}", self.side);
//     }
// }


// TODO: Define a Comparable trait
// trait Comparable {
//     fn compare(&self, other: &Self) -> Ordering;
// }


// TODO: Define a Student struct with name (String) and grade (u32)
// Add #[derive(Debug, Clone)]


// TODO: Implement Comparable for Student (compare by grade)
// impl Comparable for Student {
//     fn compare(&self, other: &Self) -> Ordering {
//         self.grade.cmp(&other.grade)
//     }
// }


// TODO: Create a generic function that prints any Drawable
// fn print_drawable<T: Drawable>(item: &T) {
//     println!("Printing drawable:");
//     item.draw();
// }


// TODO: Create a generic function to find the largest Comparable item
// fn find_largest<T: Comparable>(items: &[T]) -> &T {
//     let mut largest = &items[0];
//     for item in items.iter() {
//         if item.compare(largest) == Ordering::Greater {
//             largest = item;
//         }
//     }
//     largest
// }


fn main() {
    // TODO: Create a Circle and test Drawable
    // let circle = Circle { radius: 5 };
    // circle.draw();
    // println!("Description: {}\n", circle.describe());
    
    // TODO: Create a Square and test Drawable
    // let square = Square { side: 4 };
    // square.draw();
    // println!("Description: {}\n", square.describe());
    
    println!("---\n");
    
    // TODO: Test generic print_drawable function
    // print_drawable(&circle);
    
    println!("\n---\n");
    
    // TODO: Create students and test Comparable
    // let students = vec![
    //     Student { name: String::from("Alice"), grade: 85 },
    //     Student { name: String::from("Bob"), grade: 92 },
    //     Student { name: String::from("Charlie"), grade: 78 },
    // ];
    
    // println!("Students:");
    // for student in &students {
    //     println!("{:?}", student);
    // }
    
    // TODO: Find the best student
    // let best = find_largest(&students);
    // println!("\nBest student: {:?}", best);
    
    println!("\n---\n");
    
    // TODO: Test Display and Clone
    // println!("Circle Display: {}", circle);
    // let cloned_circle = circle.clone();
    // println!("Cloned circle: {}", cloned_circle);
}
