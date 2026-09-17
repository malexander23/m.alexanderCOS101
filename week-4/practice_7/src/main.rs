// A Rust program to make a loop of number that can satisfy a condition
// the loop cannot pass 10
use std::io;

fn main() {
    println!("Enter a number");
    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("Invalid string");

    let mut num:i32 = number
    .trim() 
    .parse()
    .expect("Input not an integer");
    println!();

    while num<10{
        println!(" inside loop number value is {}",num);
        num+=1;
    }
    println!();
    println!("Outside loop number value is {}", num);

}
