use std::f64::consts::PI;

// TODO: Define Animal trait
// trait Animal {
//     fn speak(&self);
//     fn name(&self) -> &str;
// }


// TODO: Define Dog struct with name field
// struct Dog {
//     name: String,
// }

// TODO: Implement Dog constructor and Animal trait
// impl Dog {
//     fn new(name: &str) -> Self {
//         Dog { name: name.to_string() }
//     }
// }
// 
// impl Animal for Dog {
//     fn speak(&self) {
//         println!("{} says: Woof!", self.name);
//     }
//     
//     fn name(&self) -> &str {
//         &self.name
//     }
// }


// TODO: Define Cat struct with name field


// TODO: Implement Cat constructor and Animal trait
// (speak should print "Meow!")


// TODO: Define Bird struct with name field


// TODO: Implement Bird constructor and Animal trait
// (speak should print "Chirp!")


// TODO: Create a function that accepts a vector of trait objects
// fn all_animals_speak(animals: &[Box<dyn Animal>]) {
//     for animal in animals {
//         animal.speak();
//     }
// }


// TODO: Define Shape trait
// trait Shape {
//     fn area(&self) -> f64;
//     fn perimeter(&self) -> f64;
// }


// TODO: Define Circle struct with radius


// TODO: Implement Shape for Circle
// Area = π * r²
// Perimeter = 2 * π * r


// TODO: Define Rectangle struct with width and height


// TODO: Implement Shape for Rectangle
// Area = width * height
// Perimeter = 2 * (width + height)


// TODO: Define Triangle struct with three sides


// TODO: Implement Shape for Triangle
// Use Heron's formula:
// s = (a + b + c) / 2
// area = sqrt(s * (s - a) * (s - b) * (s - c))
// perimeter = a + b + c


// TODO: Create a function to calculate total area
// fn calculate_total_area(shapes: &[Box<dyn Shape>]) -> f64 {
//     shapes.iter().map(|s| s.area()).sum()
// }


// TODO: Define Plugin trait
// trait Plugin {
//     fn init(&self);
//     fn execute(&self, input: &str) -> String;
// }


// TODO: Define Logger plugin


// TODO: Implement Plugin for Logger
// init: print "[Logger] Initializing..."
// execute: return "LOG: {input}"


// TODO: Define Calculator plugin


// TODO: Implement Plugin for Calculator
// init: print "[Calculator] Initializing..."
// execute: return "CALC: {input} = {char_count} chars"


// TODO: Define Formatter plugin


// TODO: Implement Plugin for Formatter
// init: print "[Formatter] Initializing..."
// execute: return "FORMAT: *** {input} ***"


fn main() {
    // TODO: Part 1 - Create animals and store in a vector of trait objects
    // let animals: Vec<Box<dyn Animal>> = vec![
    //     Box::new(Dog::new("Buddy")),
    //     Box::new(Cat::new("Whiskers")),
    //     Box::new(Bird::new("Tweety")),
    // ];
    
    // println!("All animals speak:");
    // all_animals_speak(&animals);
    
    println!("\n---\n");
    
    // TODO: Part 2 - Create shapes and calculate total area
    // let shapes: Vec<Box<dyn Shape>> = vec![
    //     Box::new(Circle { radius: 5.0 }),
    //     Box::new(Rectangle { width: 4.0, height: 6.0 }),
    //     Box::new(Triangle { side1: 3.0, side2: 4.0, side3: 5.0 }),
    // ];
    
    // println!("Calculating total area:");
    // for shape in &shapes {
    //     println!("{} area: {:.2}", 
    //         // Use type name somehow, or just print area
    //         shape.area()
    //     );
    // }
    // let total = calculate_total_area(&shapes);
    // println!("Total area: {:.2}", total);
    
    println!("\n---\n");
    
    // TODO: Part 3 - Create plugins and execute them
    // let plugins: Vec<Box<dyn Plugin>> = vec![
    //     Box::new(Logger),
    //     Box::new(Calculator),
    //     Box::new(Formatter),
    // ];
    
    // println!("Plugin system:");
    // for plugin in &plugins {
    //     plugin.init();
    // }
    
    // println!();
    // for plugin in &plugins {
    //     let result = plugin.execute("Hello World");
    //     println!("Executing plugin: {}", result);
    // }
}
