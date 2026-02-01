fn main() {
    println!("=== Ownership Examples ===\n");
    
    // Ownership move (commented out because it would error)
    // let s1 = String::from("hello");
    // let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // Error: s1 is no longer valid
    
    // Fix using clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy
    println!("s1 = {}, s2 = {}", s1, s2);  // Both valid!
    
    // Integers are copied (they implement Copy trait)
    let x = 5;
    let y = x;  // x is copied to y
    println!("x = {}, y = {}", x, y);  // Both valid because integers are on the stack
    
    println!("\n=== Borrowing Examples ===\n");
    
    // Borrowing with references
    let s = String::from("hello");
    let len = calculate_length(&s);  // Borrow s
    println!("The length of '{}' is {}.", s, len);  // s is still valid!
    
    // Multiple immutable references are OK
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    
    println!("\n=== Mutable References ===\n");
    
    // Mutable reference
    let mut s = String::from("Hello");
    println!("Before: {}", s);
    append_world(&mut s);
    println!("After: {}", s);
    
    // Demonstrating scope rules
    let mut s2 = String::from("test");
    {
        let r1 = &mut s2;
        r1.push_str(" in scope");
    }  // r1 goes out of scope here
    println!("s2: {}", s2);  // Now we can use s2 again
    
    // You cannot mix mutable and immutable references
    let mut s3 = String::from("hello");
    let r1 = &s3;  // Immutable borrow
    let r2 = &s3;  // Another immutable borrow
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this
    
    let r3 = &mut s3;  // Mutable borrow (OK because r1 and r2 are not used after)
    r3.push_str(" world");
    println!("r3: {}", r3);
}

// Function that borrows a String and returns its length
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but since it's just a reference,
   // the actual data is not dropped

// Function that takes a mutable reference and modifies the String
fn append_world(s: &mut String) {
    s.push_str(", world!");
}
