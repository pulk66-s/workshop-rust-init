// TODO: Define a BankAccount struct with PRIVATE fields
// - balance: f64
// - account_number: String
// Hint: Don't use 'pub' keyword for the fields!


// TODO: Implement methods for BankAccount
// impl BankAccount {
//     // Constructor - creates account with 0 balance
//     pub fn new(account_number: String) -> Self {
//     }
//
//     // Getter for balance (read-only access)
//     pub fn balance(&self) -> f64 {
//     }
//
//     // Getter for account_number (returns a string slice)
//     pub fn account_number(&self) -> &str {
//     }
//
//     // Deposit money - must be positive amount
//     pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
//         // Check if amount > 0
//         // If yes: add to balance and return Ok(())
//         // If no: return Err with error message
//     }
//
//     // Withdraw money - check sufficient funds
//     pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
//         // Check if amount > 0 and balance >= amount
//         // If yes: subtract from balance and return Ok(())
//         // If no: return Err with error message
//     }
// }


// TODO: Define a Temperature struct with PRIVATE field
// - celsius: f64
// Note: Temperature must be >= -273.15 (absolute zero)


// TODO: Implement methods for Temperature
// impl Temperature {
//     // Constructor from Celsius with validation
//     pub fn from_celsius(celsius: f64) -> Result<Self, String> {
//         // Check if celsius >= -273.15
//         // If yes: return Ok(Temperature { celsius })
//         // If no: return Err with error message
//     }
//
//     // Constructor from Fahrenheit
//     pub fn from_fahrenheit(fahrenheit: f64) -> Result<Self, String> {
//         // Convert: celsius = (fahrenheit - 32) * 5/9
//         // Then use from_celsius to validate
//     }
//
//     // Getter for Celsius
//     pub fn celsius(&self) -> f64 {
//     }
//
//     // Convert to Fahrenheit
//     pub fn fahrenheit(&self) -> f64 {
//         // Formula: fahrenheit = celsius * 9/5 + 32
//     }
//
//     // Convert to Kelvin
//     pub fn kelvin(&self) -> f64 {
//         // Formula: kelvin = celsius + 273.15
//     }
// }


fn main() {
    // TODO: Create a bank account
    // let mut account = BankAccount::new(String::from("1234567890"));
    
    // TODO: Print account info
    // println!("Account: {}", account.account_number());
    // println!("Initial balance: ${:.2}\n", account.balance());
    
    // TODO: Try to deposit money
    // match account.deposit(100.50) {
    //     Ok(_) => println!("Deposited $100.50"),
    //     Err(e) => println!("Error: {}", e),
    // }
    // println!("Balance: ${:.2}\n", account.balance());
    
    // TODO: Withdraw some money
    // match account.withdraw(30.25) {
    //     Ok(_) => println!("Withdrew $30.25"),
    //     Err(e) => println!("Error: {}", e),
    // }
    // println!("Balance: ${:.2}\n", account.balance());
    
    // TODO: Try to withdraw more than balance
    // match account.withdraw(100.00) {
    //     Ok(_) => println!("Withdrew $100.00"),
    //     Err(e) => println!("Error: {}", e),
    // }
    // println!("Balance: ${:.2}\n", account.balance());
    
    println!("---\n");
    
    // TODO: Create a temperature from Celsius
    // let temp = Temperature::from_celsius(25.0).unwrap();
    // println!("Temperature: {:.2}°C", temp.celsius());
    // println!("In Fahrenheit: {:.2}°F", temp.fahrenheit());
    // println!("In Kelvin: {:.2}K\n", temp.kelvin());
    
    // TODO: Create temperature from Fahrenheit
    // let temp2 = Temperature::from_fahrenheit(32.0).unwrap();
    // println!("Created from Fahrenheit: {:.2}°C\n", temp2.celsius());
    
    // TODO: Try to create invalid temperature
    // match Temperature::from_celsius(-300.0) {
    //     Ok(_) => println!("Temperature created"),
    //     Err(e) => println!("Error: {}", e),
    // }
}
