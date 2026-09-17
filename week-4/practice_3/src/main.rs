// A Rust program to calculate the area of a triangle when giving the base and height
use std::io;

fn main() {
    let mut base = String::new();
    let mut height = String::new();

    println!("\nEnter Base:");
    io::stdin()
    .read_line(&mut base)
    .expect("Not a valid string");

    let b:f32 = base
    .trim()
    .parse()
    .expect(" Not a valid base");

    println!("\nEnter Height:");
    io::stdin()
    .read_line(&mut height)
    .expect("Not a valid string");

    let h:f32 = height
    .trim()
    .parse()
    .expect(" Not a valid height");

    // this is the condition
    // the parenthesis holds the steps to be fufulied when the condition is met
    if b > 0.0 { 
        let area:f32 = (b*h)/2.0;
        println!("Area of the triangle = {}", area);
    }
}
