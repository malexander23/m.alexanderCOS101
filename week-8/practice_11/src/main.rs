fn main() {
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}",numbers);

    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1);

    let slice2 = &numbers[0..3];
    println!(" elements sliced from the first to the third  = {:?}",slice2);

    let slice3 = &numbers[2..];
    println!("elements sliced from the third till the last = {:?}",slice3);
    
    let slice4 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice4);
    
    let slice5 = &numbers[..];
    println!("elements sliced  which has all the terms = {:?}",slice5);
    

}
