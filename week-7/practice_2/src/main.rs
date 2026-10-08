use std::io;
fn checker(){
    let mut input_1= String::new();
    println!(" Please enter a character: ");
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let sa:char = input_1.trim().parse().expect("Invalid input");

    if sa>= '0' && sa <= '9'{
        println!("Character '{}' is a digit",sa);
    }
    else{
        println!("Character '{}' is not a digit",sa);

    }
}



fn main() {
    println!("Hello!, This is a program that checks if a character input contains a digit or not");
    checker()
}
