// A Rust Program to calculate the roots of a quadratic equation

use std::io; 
fn main() {
let mut input_1 = String::new(); // dont forget your close bracket
let mut input_2 = String::new();
let mut input_3 = String::new();

println!("\nEnter the coefficent of x^2 ");
io::stdin()
.read_line(&mut input_1)
.expect(" Not a valid string ");

let a:f32= input_1.trim()
.parse()
.expect(" Not a valid number");
println!(" \nThe value of the coeficient of x^2 is {} ",a);

println!("\nEnter the coefficent of x ");
io::stdin()
.read_line(&mut input_2) // never forget it is &mut together
.expect(" Not a valid string ");

let b:f32= input_2.trim()
.parse()
.expect(" Not a valid number");
println!(" \nThe value of the coeficient of x is {} ",b);

println!("\nEnter the constant ");
io::stdin()
.read_line(&mut input_3)
.expect(" Not a valid string ");

let c:f32= input_3.trim()
.parse()
.expect(" Not a valid number");
println!(" \nThe value of the constant {} ",c);

let  d = (b*b)-4.0*a*c;
println!("The discriminant is {}", d);

if d>0.0 {
    println!();
    println!(" There are 2 distinct roots")
}
 else if d==0.0{
    println!();
    println!(" There is only 1 real root")
}
 else if d<0.0{
    println!();
    println!("There are no real roots")
}


//All mighty formula = (-b+-(b^2 -(4ac))^-1/2)/2a
//Rust does not have plus or minus. You have to seperate the equations into 2


println!();
if d<0.0{ 
let d_1 = (d*-1.0).sqrt();// This represents the (b^2 - (4ac))^1/2
// This is done to prevent _d_1 = NaN
println!(" d_1 is {}",d_1);
println!();

// You have to split the real part and the part with imaginary number i
// This is used when d is negative
// For the first root
let firstroot_1 = -b/2.0*a;// This is for the real part of the first root
let firstroot_2 =d_1/2.0*a; // This is for the imaginary part of the first root
println!(" The first root is {} + {}i",firstroot_1,firstroot_2);
// this prevents the value of the first root showing NaN
println!();

// For the second root
let secondroot_1 = -b/2.0*a;// This is for the real part of the second root
let secondroot_2 =d_1/2.0*a; // This is for the imaginary part of the second root
println!(" The second root is {} - {}i",secondroot_1,secondroot_2);
// this prevents the value of the second root showing NaN
}
else if d> 0.0{
    let d_2 = d.sqrt();// This represents the (b^2 - (4ac))^1/2
// This is used when d is positive

let first_root = (-b+ d_2)/2.0*a;
let second_root = (-b-d_2)/2.0*a;
println!("The first root is {}",first_root);
println!("The second root is {}",second_root);
}
else if d==0.0 {
    let d_2 = d.sqrt();// This represents the (b^2 - (4ac))^1/2

    let firstroot = (-b)/2.0*a;
let secondroot = (-b)/2.0*a;
println!("The first root is {}",firstroot); // the same formula because d and d_1 is zero. it disappears in solution
println!("The second root is {}",secondroot);// the same formula because d and d_1 is zero. it disappears in solution
println!(" The roots are {} twice", firstroot);// conculsion
}

//If you put your if and else if to identify _d_1 and _d_2 then quote it in another if, else if. it will not run
// This is because if it fails, there will be no _d_1 to refer to in the second part



}
