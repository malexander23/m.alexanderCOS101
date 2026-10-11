use std::io;
fn trapezium(){
    println!("\nYou have chosen trapezium as your shape.");
    println!("\nPlease enter the value of the height");
    let mut h=String::new();
    io::stdin().read_line(&mut h).expect("failed to read input");
   let h1:f32=h.trim().parse().expect("invalid input");
    
    println!("\nPlease enter the value of the first base");
    let mut b_1=String::new();
    io::stdin().read_line(&mut b_1).expect("failed to read input");
    let b_11:f32=b_1.trim().parse().expect("invalid input");

    println!("\nPlease enter the value of the second base");
    let mut b_2=String::new();
    io::stdin().read_line(&mut b_2).expect("failed to read input");
    let b_21:f32=b_2.trim().parse().expect("invalid input");
      let a=(h1/2.0)*(b_11 + b_21);
    println!("area= {}",a);
    //do the lowercase

}
 
fn rhombus(){
  println!("\n You have chosen rhombus as your shape. ");
  println!("\n Please enter the value of the first diagonal");
  let mut d_1 = String::new();
  io::stdin().read_line(&mut d_1).expect("Failed to read input");
  let d_11:f32= d_1.trim().parse().expect("Invalid input");

  println!("\n Please enter the value of the second diagonal");
  let mut d_2 = String::new();
  io::stdin().read_line(&mut d_2).expect("Failed to read input");
  let d_21:f32= d_2.trim().parse().expect("Invalid input");
  let a_1 = (1.0/2.0) *d_11 * d_21;
  println!("\nArea = {}",a_1);
}

fn parallelogram(){
  println!("\n You have chosen parallelogram as yor shape");
  println!("\n Please enter the value of the base");
  let mut b=String::new();
  io::stdin().read_line(&mut b).expect("Failed to read input");
 let  b1:f32 = b.trim().parse().expect("Invalid input");
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Failed to read input");
   let a1:f32 = a.trim().parse().expect("Invalid input");
  let a_2 = b1 * a1;
 println!("Area = {}",a_2);

}

fn cube(){
  println!("\n You have chosen a cube");
  println!("\n This program calculates the surface area");
  println!("\n Please enter the value of the side");
  let mut s_1 =String::new();
  io::stdin().read_line(&mut s_1).expect("Failed to read input");
  let s_11:f32= s_1.trim().parse().expect("Invalid input");
  let a_3 = s_11*s_11*6.0;
  println!("Surface Area = {}",a_3);


}
fn cylinder (){
  println!("\n You have chosen a cyliner");
  println!("\n Please enter the value of the radius");
  let mut r= String::new();
  io::stdin().read_line(&mut r).expect("Failed to read input");
  let r1:f32= r.trim().parse().expect("Invalid input");
  let pi:f32 = 3.142857143;
  println!("\n Please enter the value of the height");
  let mut  h_1 = String::new();
   io::stdin().read_line(&mut h_1).expect("Failed to read input");
  let h_11:f32= h_1.trim().parse().expect("Invalid input");
  let a_4 = pi * h_11*r1 * r1;
  println!("Volume = {}",a_4);
}


fn main() { 
    println!("Welcome to a programme that calculates the area,volume,surface area of shapes");
    println!("The avaialable shapes this programme can calculate: trapezium rhombus parallelogram cube cylinder");
  println!("Enter the shape you want to calculate the area for");
 
    
    let mut s =String::new();
    io::stdin().read_line(&mut s).expect("failed to read input");
    if s.trim().to_lowercase()=="trapezium" {
    trapezium();
    }

    if s.trim().to_lowercase()=="rhombus" {
    rhombus();
    }

     if  s.trim().to_lowercase()=="parallelogram" {
    parallelogram();
    }

     if  s.trim().to_lowercase()=="cube" {
    cube();
    }

     if s.trim().to_lowercase()=="cylinder" {
    cylinder();
    }

    
    }
   