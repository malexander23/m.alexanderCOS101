// Rust program to check the party attendance age requirement

use std::io;


fn main() {
    let mut name = String::new();
    let mut age = String::new();

    // type name
    println!("\n Please enter your name.");
    io::stdin()  
    .read_line(&mut name)
    .expect("Not a vaild string"); // shows where an error occurs

    // type age
    println!("\nPlease enter your age.");
    io::stdin()
    .read_line(&mut age)
    .expect("Invalid string");
    
    let age:f32 = age
    .trim()
    .parse()
    .expect(" Not a valid number");
// this is when it check if you are up to the age

    if age >= 18.0 {
    println!("You are welcome to the party {}", name) ;  
    } else {
        println!(" Oops sorry {}, Your age does not meet the requirement", name);
    }
}
