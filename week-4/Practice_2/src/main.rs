// A Rust Program to calculate the area of a triangle 
use std::io;


fn main() {
    println!();

    println!("NOTE: The sum of any 2 side of the triangle must be greater than the last side");
    println!();
    let mut first_side = String::new();
    let mut second_side = String::new();
    let mut third_side = String::new();


    println!(" Enter the lenght of the first side");
    io::stdin()
    .read_line(&mut first_side)
    .expect(" Not a valid string for first_side");

    let a:f32 = first_side // the first value inputted is taken as a
    .trim() // removes unnecesary stuff
    .parse() // converts first_side that was trimmed to a  float number
    .expect("This is not a valid number value for a side");

    println!(" Enter the lenght of the second side");
    io::stdin()
    .read_line(&mut second_side)
    .expect(" Not a valid string for second_side");

    let b:f32 = second_side // the second value inputted is taken as b
    .trim() // removes unnecesary stuff
    .parse() // converts second_side that was trimmed  to a float number
    .expect("This is not a valid number value for a side");

    println!(" Enter the lenght of the third side");
    io::stdin()
    .read_line(&mut third_side)
    .expect(" Not a valid string for third_side");

    let c:f32 = third_side // the third value inputted is taken as c
    .trim() // removes unnecesary stuff
    .parse() // converts third_side that was trimmed to a  float number
    .expect("This is not a valid number value for a side");

   

    let s: f32 = (a+b+c)/2.0;
    let mut area:f32 = s* (s-a)*(s-b)*(s-c);
    area = area.sqrt();

    println!( "Area of the triangle = {}", area);


}

