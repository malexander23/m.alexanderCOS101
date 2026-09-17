fn main() {
    let mut count= 0; // it has to start from 0
    // the 1..21 gives a range
    for num in 1..21 {
        if num> 10{
            println!("{:?}",num);
            continue;// it continues only when it passes the condition
        }
        count+=1
    }
    println!("The number of values greater than 10 ( between 1 and 20) is:{}",count);// the total numbers that were counted or how many numbers were  output
}
