fn main() {
    println!("=== Ownership Examples ===\n");
    
    // TODO: Observe ownership move
    // Uncomment these lines and see what happens:
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1);  // This will error! Why?
    
    
    // TODO: Fix the above by using clone()
    
    
    // TODO: Demonstrate that integers (stack types) are copied, not moved
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);  // This works! Why?
    
    
    println!("\n=== Borrowing Examples ===\n");
    
    // TODO: Create a function that takes a reference and returns the length
    // fn calculate_length(s: &String) -> usize { ... }
    
    
    // TODO: Call the function with a reference
    // let s = String::from("hello");
    // let len = calculate_length(&s);
    // println!("The length of '{}' is {}.", s, len);
    
    
    println!("\n=== Mutable References ===\n");
    
    // TODO: Create a function that modifies a string
    // fn append_world(s: &mut String) { ... }
    
    
    // TODO: Create a mutable String and modify it
    // let mut s = String::from("Hello");
    // append_world(&mut s);
    
    
    // TODO: Demonstrate that you cannot have multiple mutable references
    // Uncomment to see the error:
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;  // Error!
    // println!("{}, {}", r1, r2);
    
}

// TODO: Implement calculate_length function here


// TODO: Implement append_world function here
