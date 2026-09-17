
// A Rust Program to show name and age

use std::io; // Always type it
fn main() {
    println!("\nStudent Information Management System");

    //type name
    println!("\n Please enter your name.");
    let mut full_name = String::new();
    io::stdin()  
    .read_line(&mut full_name)
    .expect("Failed to read input"); // shows where an error occurs

    println!("Your name is : {}", full_name);

    // type age
    println!("\nPlease Enter your age.");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("Failed to read input");

    let age:i8 = age
    .trim() // for removing whitespaces and new line
    .parse() // for converting the text to a number
    .expect("Input not an integer");
    println!("Your age is: {}", age);


}
