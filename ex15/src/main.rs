// TODO: Part 1 - Builder Pattern
// Define User struct with fields: username, email, age, bio
// struct User {
//     username: String,
//     email: Option<String>,
//     age: Option<u32>,
//     bio: Option<String>,
// }

// TODO: Define UserBuilder struct
// struct UserBuilder {
//     username: String,
//     email: Option<String>,
//     age: Option<u32>,
//     bio: Option<String>,
// }

// TODO: Implement UserBuilder
// impl UserBuilder {
//     pub fn new(username: &str) -> Self {
//         // Initialize with username, rest None
//     }
//
//     pub fn email(mut self, email: &str) -> Self {
//         self.email = Some(email.to_string());
//         self
//     }
//
//     pub fn age(mut self, age: u32) -> Self {
//         self.age = Some(age);
//         self
//     }
//
//     pub fn bio(mut self, bio: &str) -> Self {
//         self.bio = Some(bio.to_string());
//         self
//     }
//
//     pub fn build(self) -> Result<User, String> {
//         // Validate email if present (must contain '@')
//         // Return Ok(User) or Err(String)
//     }
// }

// TODO: Implement Display for User
// impl std::fmt::Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "User: {}", self.username)?;
//         if let Some(email) = &self.email {
//             write!(f, " ({})", email)?;
//         }
//         if let Some(age) = self.age {
//             write!(f, ", age {}", age)?;
//         }
//         Ok(())
//     }
// }


// TODO: Part 2 - Composition
// Define Engine struct with horsepower and fuel_type
// struct Engine {
//     horsepower: u32,
//     fuel_type: String,
// }

// TODO: Implement Engine methods
// impl Engine {
//     pub fn new(horsepower: u32, fuel_type: &str) -> Self {
//         // Create engine
//     }
//
//     pub fn start(&self) {
//         println!("Engine started!");
//     }
//
//     pub fn stop(&self) {
//         println!("Engine stopped!");
//     }
// }

// TODO: Define Car struct that CONTAINS an Engine
// struct Car {
//     model: String,
//     engine: Engine,
//     mileage: u32,
// }

// TODO: Implement Car methods
// impl Car {
//     pub fn new(model: &str, engine: Engine) -> Self {
//         // Create car with 0 mileage
//     }
//
//     pub fn start(&self) {
//         println!("Starting the car...");
//         self.engine.start();  // Delegate to engine
//     }
//
//     pub fn stop(&self) {
//         println!("Stopping the car...");
//         self.engine.stop();  // Delegate to engine
//     }
//
//     pub fn info(&self) {
//         println!("Car: {}", self.model);
//         println!("Engine: {}hp, {}", self.engine.horsepower, self.engine.fuel_type);
//     }
// }


// TODO: Part 3 - Newtype Pattern
// Create Email newtype that wraps String
// struct Email(String);

// TODO: Implement Email with validation
// impl Email {
//     pub fn new(email: &str) -> Result<Self, String> {
//         // Check if email contains '@'
//         // If yes: Ok(Email(email.to_string()))
//         // If no: Err("Invalid email format")
//     }
//
//     pub fn as_str(&self) -> &str {
//         &self.0
//     }
// }


// TODO: Part 4 - Strategy Pattern
// Define SortStrategy trait
// trait SortStrategy {
//     fn sort(&self, data: &mut Vec<i32>);
// }

// TODO: Define BubbleSort struct (empty, just a marker)
// struct BubbleSort;

// TODO: Implement SortStrategy for BubbleSort
// impl SortStrategy for BubbleSort {
//     fn sort(&self, data: &mut Vec<i32>) {
//         // Implement bubble sort algorithm
//         let len = data.len();
//         for i in 0..len {
//             for j in 0..len - 1 - i {
//                 if data[j] > data[j + 1] {
//                     data.swap(j, j + 1);
//                 }
//             }
//         }
//     }
// }

// TODO: Define QuickSort struct
// struct QuickSort;

// TODO: Implement SortStrategy for QuickSort
// impl SortStrategy for QuickSort {
//     fn sort(&self, data: &mut Vec<i32>) {
//         // Simple quick sort implementation
//         if data.len() <= 1 {
//             return;
//         }
//         data.sort();  // Use standard library for simplicity
//     }
// }

// TODO: Define Sorter that uses a strategy
// struct Sorter {
//     strategy: Box<dyn SortStrategy>,
// }

// TODO: Implement Sorter
// impl Sorter {
//     pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
//         Sorter { strategy }
//     }
//
//     pub fn sort(&self, data: &mut Vec<i32>) {
//         self.strategy.sort(data);
//     }
// }


fn main() {
    println!("=== Builder Pattern ===");
    // TODO: Create users with builder
    // let user1 = UserBuilder::new("alice")
    //     .email("alice@example.com")
    //     .age(25)
    //     .build()
    //     .unwrap();
    // println!("User created: {}", user1);
    
    // TODO: Create user with minimal info
    // let user2 = UserBuilder::new("bob").build().unwrap();
    // println!("User with minimal info: {}\n", user2);
    
    // TODO: Try to create user with invalid email
    // match UserBuilder::new("charlie")
    //     .email("not-an-email")
    //     .build()
    // {
    //     Ok(user) => println!("User created: {}", user),
    //     Err(e) => println!("Error creating user: {}\n", e),
    // }
    
    println!("\n=== Composition ===");
    // TODO: Create a car with an engine
    // let engine = Engine::new(1020, "Electric");
    // let car = Car::new("Tesla Model S", engine);
    // car.info();
    // car.start();
    // car.stop();
    
    println!("\n=== Newtype Pattern ===");
    // TODO: Create valid email
    // match Email::new("alice@example.com") {
    //     Ok(email) => println!("Valid email: {}", email.as_str()),
    //     Err(e) => println!("Error: {}", e),
    // }
    
    // TODO: Try invalid email
    // match Email::new("not-an-email") {
    //     Ok(email) => println!("Valid email: {}", email.as_str()),
    //     Err(e) => println!("Error: {}\n", e),
    // }
    
    println!("\n=== Strategy Pattern ===");
    // TODO: Test different sorting strategies
    // let mut numbers = vec![5, 2, 8, 1, 9];
    // println!("Original: {:?}", numbers);
    
    // let sorter = Sorter::new(Box::new(BubbleSort));
    // let mut data = numbers.clone();
    // sorter.sort(&mut data);
    // println!("Bubble sorted: {:?}", data);
    
    // let sorter = Sorter::new(Box::new(QuickSort));
    // let mut data = numbers.clone();
    // sorter.sort(&mut data);
    // println!("Quick sorted: {:?}", data);
}
