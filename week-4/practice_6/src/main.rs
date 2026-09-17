// A Rust Program to count numbers when given a range
use std::io;

fn main() {
    println!("Please enter the lower_boundary"); // registers the starting number
    let mut lower_boundary = String::new();
    io::stdin()
    .read_line(&mut lower_boundary)
    .expect("Invalid input");

    let a:i32 = lower_boundary
    .trim() 
    .parse()
    .expect("Input not an integer");

    println!("Please enter the upper_boundary"); // registers the number after the last number the rust program will count
    let mut upper_boundary = String::new();
    io::stdin()
    .read_line(&mut upper_boundary)
    .expect("Invalid input");

    let b:i32 = upper_boundary
    .trim() 
    .parse()
    .expect("Input not an integer");
// upper_boundary is not added
    for m in a..b{  
        println!("Count level is {}",m);
    }
}
