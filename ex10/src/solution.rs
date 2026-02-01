use std::fmt;

// Define a Summary trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Article struct
#[derive(Debug, Clone)]
struct Article {
    title: String,
    content: String,
    author: String,
}

// Implement Summary for Article
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Tweet struct
#[derive(Debug, Clone)]
struct Tweet {
    username: String,
    message: String,
    likes: u32,
}

// Implement Summary for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.message)
    }
}

// Function that accepts any type implementing Summary
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Alternative syntax with trait bounds
fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

// Animal trait
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
}

// Dog struct
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}

// Cat struct
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn speak(&self) -> String {
        String::from("Meow!")
    }
}

// Function with trait bound
fn make_noise<T: Animal>(animal: &T) {
    println!("{} says: {}", animal.name(), animal.speak());
}

// Function that takes multiple trait bounds
fn describe<T: Animal + fmt::Debug>(animal: &T) {
    println!("Animal: {:?}", animal);
    println!("{} says {}", animal.name(), animal.speak());
}

// Implement Display for Article
impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Article: '{}' by {}", self.title, self.author)
    }
}

// Implement Display for Tweet
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tweet from @{}: {} ({} likes)", 
               self.username, self.message, self.likes)
    }
}

// Implement Debug for Dog and Cat manually
impl fmt::Debug for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog {{ name: {} }}", self.name)
    }
}

impl fmt::Debug for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cat {{ name: {} }}", self.name)
    }
}

// Trait with associated types
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}

struct Box<T> {
    value: T,
}

impl<T> Container for Box<T> {
    type Item = T;
    
    fn get(&self) -> &Self::Item {
        &self.value
    }
}

fn main() {
    println!("=== Summary Trait ===");
    
    // Create Article instance
    let article = Article {
        title: String::from("Rust Programming"),
        content: String::from("Rust is a systems programming language..."),
        author: String::from("Alice"),
    };
    
    // Create Tweet instance
    let tweet = Tweet {
        username: String::from("bob_coder"),
        message: String::from("Loving Rust! #rustlang"),
        likes: 42,
    };
    
    // Call summarize
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Default: {}", article.default_summary());
    
    // Pass to notify function
    notify(&article);
    notify(&tweet);
    
    // Use alternative syntax
    print_summary(&article);
    print_summary(&tweet);
    
    println!("\n=== Animal Trait ===");
    
    // Create Dog and Cat instances
    let dog = Dog {
        name: String::from("Buddy"),
    };
    
    let cat = Cat {
        name: String::from("Whiskers"),
    };
    
    // Call make_noise
    make_noise(&dog);
    make_noise(&cat);
    
    // Direct calls
    println!("{} is a dog", dog.name());
    println!("{} is a cat", cat.name());
    
    println!("\n=== Standard Library Traits ===");
    
    // Display trait
    println!("Display: {}", article);
    println!("Display: {}", tweet);
    
    // Debug trait
    println!("Debug: {:?}", article);
    println!("Debug: {:?}", tweet);
    println!("Debug: {:?}", dog);
    println!("Debug: {:?}", cat);
    
    // Clone trait
    let article2 = article.clone();
    println!("Cloned article: {}", article2);
    
    // Using describe function with multiple bounds
    println!("\n=== Multiple Trait Bounds ===");
    describe(&dog);
    describe(&cat);
    
    println!("\n=== Associated Types ===");
    let int_box = Box { value: 42 };
    let string_box = Box { value: String::from("hello") };
    
    println!("Int box contains: {}", int_box.get());
    println!("String box contains: {}", string_box.get());
}
