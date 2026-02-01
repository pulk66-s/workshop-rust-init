// Define BankAccount with private fields
pub struct BankAccount {
    balance: f64,
    account_number: String,
}

impl BankAccount {
    // Constructor - creates account with 0 balance
    pub fn new(account_number: String) -> Self {
        BankAccount {
            balance: 0.0,
            account_number,
        }
    }

    // Getter for balance (read-only access)
    pub fn balance(&self) -> f64 {
        self.balance
    }

    // Getter for account_number (returns a string slice)
    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    // Deposit money - must be positive amount
    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Deposit amount must be positive"));
        }
        self.balance += amount;
        Ok(())
    }

    // Withdraw money - check sufficient funds
    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Withdrawal amount must be positive"));
        }
        if self.balance < amount {
            return Err(String::from("Insufficient funds"));
        }
        self.balance -= amount;
        Ok(())
    }
}

// Define Temperature with private field
pub struct Temperature {
    celsius: f64,
}

impl Temperature {
    const ABSOLUTE_ZERO: f64 = -273.15;

    // Constructor from Celsius with validation
    pub fn from_celsius(celsius: f64) -> Result<Self, String> {
        if celsius < Self::ABSOLUTE_ZERO {
            return Err(format!(
                "Temperature cannot be below absolute zero ({:.2}°C)",
                Self::ABSOLUTE_ZERO
            ));
        }
        Ok(Temperature { celsius })
    }

    // Constructor from Fahrenheit
    pub fn from_fahrenheit(fahrenheit: f64) -> Result<Self, String> {
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        Self::from_celsius(celsius)
    }

    // Getter for Celsius
    pub fn celsius(&self) -> f64 {
        self.celsius
    }

    // Convert to Fahrenheit
    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    // Convert to Kelvin
    pub fn kelvin(&self) -> f64 {
        self.celsius + 273.15
    }
}

fn main() {
    // Create a bank account
    let mut account = BankAccount::new(String::from("1234567890"));
    
    // Print account info
    println!("Account: {}", account.account_number());
    println!("Initial balance: ${:.2}\n", account.balance());
    
    // Deposit money
    match account.deposit(100.50) {
        Ok(_) => println!("Deposited $100.50"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Balance: ${:.2}\n", account.balance());
    
    // Withdraw some money
    match account.withdraw(30.25) {
        Ok(_) => println!("Withdrew $30.25"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Balance: ${:.2}\n", account.balance());
    
    // Try to withdraw more than balance
    match account.withdraw(100.00) {
        Ok(_) => println!("Withdrew $100.00"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Balance: ${:.2}\n", account.balance());
    
    println!("---\n");
    
    // Create a temperature from Celsius
    let temp = Temperature::from_celsius(25.0).unwrap();
    println!("Temperature: {:.2}°C", temp.celsius());
    println!("In Fahrenheit: {:.2}°F", temp.fahrenheit());
    println!("In Kelvin: {:.2}K\n", temp.kelvin());
    
    // Create temperature from Fahrenheit
    let temp2 = Temperature::from_fahrenheit(32.0).unwrap();
    println!("Created from Fahrenheit: {:.2}°C\n", temp2.celsius());
    
    // Try to create invalid temperature
    match Temperature::from_celsius(-300.0) {
        Ok(_) => println!("Temperature created"),
        Err(e) => println!("Error: {}", e),
    }
}
