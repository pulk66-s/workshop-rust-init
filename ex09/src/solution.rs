use std::collections::HashMap;

// Function to count word frequencies
fn word_frequency(text: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    
    map
}

// Function to find the maximum value in a vector
fn find_max(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        None
    } else {
        Some(*vec.iter().max().unwrap())
    }
}

fn main() {
    println!("=== Vectors ===");
    
    // Create an empty vector
    let mut numbers: Vec<i32> = Vec::new();
    
    // Push values
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    
    println!("Vector: {:?}", numbers);
    
    // Access element by index
    let third = numbers[2];
    println!("Third element: {}", third);
    
    // Safe access with get()
    match numbers.get(2) {
        Some(value) => println!("Element at index 2: {}", value),
        None => println!("No element at index 2"),
    }
    
    match numbers.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("No element at index 10"),
    }
    
    // Iterate over vector
    print!("Elements: ");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();
    
    // Pop last element
    if let Some(last) = numbers.pop() {
        println!("Popped: {}", last);
    }
    println!("After pop: {:?}", numbers);
    
    // Create vector with macro
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.push("date");
    println!("Fruits: {:?}", fruits);
    
    println!("\n=== HashMap ===");
    
    // Create a HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Green"), 30);
    
    println!("Scores: {:?}", scores);
    
    // Access a value
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("{} team score: {}", team_name, score),
        None => println!("Team not found"),
    }
    
    // Update a value
    scores.insert(String::from("Blue"), 25);
    println!("After update: {:?}", scores);
    
    // Use entry API
    scores.entry(String::from("Yellow")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(100);  // Won't change Blue
    println!("After entry operations: {:?}", scores);
    
    // Iterate over HashMap
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // Update based on old value
    let text = "hello world hello rust";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:?}", word_count);
    
    println!("\n=== Strings ===");
    
    // Create a String
    let mut s = String::from("Hello");
    println!("Initial string: {}", s);
    
    // Append to String
    s.push_str(", world");
    println!("After push_str: {}", s);
    
    s.push('!');
    println!("After push: {}", s);
    
    // Concatenate strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved here
    println!("Concatenated: {}", s3);
    
    // Using format! macro
    let name = "Alice";
    let age = 25;
    let message = format!("{} is {} years old", name, age);
    println!("{}", message);
    
    // String slicing
    let hello = String::from("Hello, world!");
    let hello_slice = &hello[0..5];
    println!("Slice: {}", hello_slice);
    
    println!("\n=== Challenge: Word Frequency ===");
    
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let frequencies = word_frequency(text);
    
    println!("Word frequencies:");
    let mut words: Vec<_> = frequencies.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1));  // Sort by frequency
    
    for (word, count) in words {
        println!("  '{}': {}", word, count);
    }
    
    // Bonus: Find max in vector
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    match find_max(&nums) {
        Some(max) => println!("\nMax value: {}", max),
        None => println!("\nVector is empty"),
    }
}
