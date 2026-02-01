// TODO: Define a trait called Summary with a method summarize() that returns String


// TODO: Define a struct Article with title and content fields


// TODO: Implement Summary for Article


// TODO: Define a struct Tweet with username and message fields


// TODO: Implement Summary for Tweet


// TODO: Create a function that accepts any type implementing Summary
// fn notify(item: &impl Summary)


// TODO: Define a trait Animal with methods name() and speak()


// TODO: Implement Animal for Dog and Cat structs


// TODO: Create a function that takes a trait bound
// fn make_noise<T: Animal>(animal: &T)


use std::fmt;

// TODO: Implement Display trait for one of your structs


fn main() {
    println!("=== Summary Trait ===");
    
    // TODO: Create Article and Tweet instances
    
    
    // TODO: Call summarize on each
    
    
    // TODO: Pass them to notify function
    
    
    println!("\n=== Animal Trait ===");
    
    // TODO: Create Dog and Cat instances
    
    
    // TODO: Call make_noise on each
    
    
    println!("\n=== Standard Library Traits ===");
    
    // TODO: Demonstrate Debug and Display traits
    
}
