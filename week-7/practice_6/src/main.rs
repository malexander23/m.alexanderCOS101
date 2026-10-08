fn main() {
    let mut num : i32 =5;
    println!(" num is {}",num);  // this shows the actual number put first
    mutate_num(&mut num); // the function is called
    println!(" The value of num after is: {}", num);// it prints the num after it is updated

}
fn mutate_num(num_1:&mut i32){           // this shows the value of num_1 is mutable and integer 32
    *num_1 = *num_1*2;                  // the left hand side is what updates num_1, the right hand finds the value of num_1 stated previously
    println!(" The value of num_1 is : {}",num_1);  // shows the value after the operation
}
// * is used when referencing