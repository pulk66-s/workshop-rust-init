fn main() {
    // If/else to check if a number is positive, negative, or zero
    let number = -5;
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is zero", number);
    }
    
    // If as an expression
    let number = 6;
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, result);
    
    // Loop to calculate factorial of 5
    let mut factorial = 1;
    let mut counter = 5;
    loop {
        if counter == 0 {
            break;
        }
        factorial *= counter;
        counter -= 1;
    }
    println!("Factorial of 5 is: {}", factorial);
    
    // While loop countdown
    let mut countdown = 5;
    print!("Countdown: ");
    while countdown > 0 {
        print!("{} ", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");
    
    // For loop from 1 to 5
    print!("Numbers 1 to 5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    // For loop over an array
    let numbers = [10, 20, 30, 40, 50];
    print!("Array elements: ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();
    
    // Match to classify a number
    let digit = 2;
    let word = match digit {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("The digit {} is: {}", digit, word);
    
    // Match with ranges
    let number = 7;
    let size = match number {
        1..=5 => "small",
        6..=10 => "medium",
        _ => "large",
    };
    println!("{} is {}", number, size);
    
    // Bonus: continue in a loop
    println!("Even numbers from 1 to 10:");
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;  // Skip odd numbers
        }
        print!("{} ", i);
    }
    println!();
}
