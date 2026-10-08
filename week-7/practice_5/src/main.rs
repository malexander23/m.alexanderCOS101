fn main() {
    let num: i32 =5;
    abc(num);
    println!(" The value of no is: {}", num);

}

fn abc(mut num_1:i32){
    num_1 = num_1*0;
    println!(" The value of num_1 is : {}", num_1);
}
