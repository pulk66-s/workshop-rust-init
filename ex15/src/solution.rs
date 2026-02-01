// Part 1 - Builder Pattern
#[derive(Debug)]
struct User {
    username: String,
    email: Option<String>,
    age: Option<u32>,
    bio: Option<String>,
}

struct UserBuilder {
    username: String,
    email: Option<String>,
    age: Option<u32>,
    bio: Option<String>,
}

impl UserBuilder {
    pub fn new(username: &str) -> Self {
        UserBuilder {
            username: username.to_string(),
            email: None,
            age: None,
            bio: None,
        }
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn bio(mut self, bio: &str) -> Self {
        self.bio = Some(bio.to_string());
        self
    }

    pub fn build(self) -> Result<User, String> {
        // Validate email if present
        if let Some(ref email) = self.email {
            if !email.contains('@') {
                return Err(String::from("Invalid email format"));
            }
        }

        Ok(User {
            username: self.username,
            email: self.email,
            age: self.age,
            bio: self.bio,
        })
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)?;
        if let Some(email) = &self.email {
            write!(f, " ({})", email)?;
        }
        if let Some(age) = self.age {
            write!(f, ", age {}", age)?;
        }
        Ok(())
    }
}

// Part 2 - Composition
struct Engine {
    horsepower: u32,
    fuel_type: String,
}

impl Engine {
    pub fn new(horsepower: u32, fuel_type: &str) -> Self {
        Engine {
            horsepower,
            fuel_type: fuel_type.to_string(),
        }
    }

    pub fn start(&self) {
        println!("Engine started!");
    }

    pub fn stop(&self) {
        println!("Engine stopped!");
    }
}

struct Car {
    model: String,
    engine: Engine,
    mileage: u32,
}

impl Car {
    pub fn new(model: &str, engine: Engine) -> Self {
        Car {
            model: model.to_string(),
            engine,
            mileage: 0,
        }
    }

    pub fn start(&self) {
        println!("Starting the car...");
        self.engine.start();
    }

    pub fn stop(&self) {
        println!("Stopping the car...");
        self.engine.stop();
    }

    pub fn info(&self) {
        println!("Car: {}", self.model);
        println!("Engine: {}hp, {}", self.engine.horsepower, self.engine.fuel_type);
    }
}

// Part 3 - Newtype Pattern
struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Email(email.to_string()))
        } else {
            Err(format!("Invalid email format: {}", email))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Part 4 - Strategy Pattern
trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        if data.len() <= 1 {
            return;
        }
        // Using standard library sort for simplicity
        data.sort();
    }
}

struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Sorter { strategy }
    }

    pub fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}

fn main() {
    println!("=== Builder Pattern ===");
    // Create users with builder
    let user1 = UserBuilder::new("alice")
        .email("alice@example.com")
        .age(25)
        .build()
        .unwrap();
    println!("User created: {}", user1);
    
    // Create user with minimal info
    let user2 = UserBuilder::new("bob").build().unwrap();
    println!("User with minimal info: {}\n", user2);
    
    // Try to create user with invalid email
    match UserBuilder::new("charlie")
        .email("not-an-email")
        .build()
    {
        Ok(user) => println!("User created: {}", user),
        Err(e) => println!("Error creating user: {}", e),
    }
    
    println!("\n=== Composition ===");
    // Create a car with an engine
    let engine = Engine::new(1020, "Electric");
    let car = Car::new("Tesla Model S", engine);
    car.info();
    car.start();
    car.stop();
    
    println!("\n=== Newtype Pattern ===");
    // Create valid email
    match Email::new("alice@example.com") {
        Ok(email) => println!("Valid email: {}", email.as_str()),
        Err(e) => println!("Error: {}", e),
    }
    
    // Try invalid email
    match Email::new("not-an-email") {
        Ok(email) => println!("Valid email: {}", email.as_str()),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Strategy Pattern ===");
    // Test different sorting strategies
    let numbers = vec![5, 2, 8, 1, 9];
    println!("Original: {:?}", numbers);
    
    let sorter = Sorter::new(Box::new(BubbleSort));
    let mut data = numbers.clone();
    sorter.sort(&mut data);
    println!("Bubble sorted: {:?}", data);
    
    let sorter = Sorter::new(Box::new(QuickSort));
    let mut data = numbers.clone();
    sorter.sort(&mut data);
    println!("Quick sorted: {:?}", data);
}
