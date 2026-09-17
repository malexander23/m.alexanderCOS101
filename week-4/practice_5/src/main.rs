// A Rust Program to check your height category

use std::io; // always put it

fn main() {
    let mut height = String::new();

    // type height
    println!("\n Please enter your Height (in centimeters): ");
    io::stdin()
    .read_line(&mut height)
    .expect("Invalid string");
    
    let h:f32 = height
    .trim()
    .parse()
    .expect(" Not a valid height in cm");
// this are the height conditions with 4 differnt possible results
    if h >= 150.00 && h<= 170.00{
        println!(" You are an average-height person");
    }
    else if h > 170.00 && h<= 195.00 {
        println!(" You are a tall person");
    }
    else if h > 100.00 && h< 150.00 {
        println!(" You are a dwarf person");
    }
    else{
        println!(" Abnormal height");
    }

}
