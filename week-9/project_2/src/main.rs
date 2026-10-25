use std::io;
use std::io::Write;

fn main(){
    println!("Welcome, This iss a program that helps to backup student data\n");

let mut count=0;
let stud=String::new();
let mut name: Vec<String> =Vec::new();
let mut matr:Vec<String> = Vec::new();
let mut dep:Vec<String> = Vec::new();
let mut lvl:Vec<String> = Vec::new();


    println!("How many students records are going to be inputted\n input a whole number");
let mut a1=String::new();
io::stdin().read_line(&mut a1).expect("Failed to read input");
let a:u8=a1.trim().parse().expect("Invalid input");
loop{
  println!("Student {}'s Data form\n",count+1);
  println!("Please enter the student's name");
  let mut stu=String::new();
  io::stdin().read_line(&mut stu).expect("Failed to read input");
  let stud = stu.trim().to_string();
  name.push(stud.clone());
  println!();

   println!("Please enter the student's matriculation number");
  let mut matn=String::new();
  io::stdin().read_line(&mut matn).expect("Failed to read input");
  let mat = matn.trim().to_string();
  matr.push(mat.clone());
  println!();

   println!("Please enter the student's department");
  let mut depar=String::new();
  io::stdin().read_line(&mut depar).expect("Failed to read input");
  let depa = depar.trim().to_string();
  dep.push(depa.clone());
  println!();

   println!("Please enter the student's level");
   println!("eg 100, 200 etc");
  let mut lev=String::new();
  io::stdin().read_line(&mut lev).expect("Failed to read input");
  let leve = lev.trim().to_string();
  lvl.push(leve);
  println!();

  count+=1;
  if a==count{
    break
  }
}
println!();
for i in 0..name.len(){
println!("The student's name is {}. Matric No:{}.Department:{}. Level:{}",name[i],matr[i],dep[i],lvl[1]);
}
  
let mut backup = std::fs::File::create("Student_backup.csv").expect("Failed to create file");
backup.write_all("PAU's Student Management Information System\n".as_bytes()).expect("Failed to write line");
backup.write_all("\n".as_bytes()).expect("Failed to write empty line");
backup.write_all("Student Name,Matric. Number,Department,Level".as_bytes()).expect("Failed to write heading");

for i in 0..name.len(){
    backup.write_all(format!("\n{},{},{},{}\n",name[i],matr[i],dep[i],lvl[i]).as_bytes()).expect("Failed to write line");
}

}

\\for i in 1..comm_1.len(){}
