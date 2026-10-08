fn main() {
    // array when it is stated
    let arr1 : [i32;4] = [10,20,30,40];  // first is the type of the integer, the second is the number of values
    println!("\n Array with data type");
    println!("\n array is {:?}",arr1);
    println!(" The size of the array is : {}",arr1.len());
    
// when array is not stated
    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\n Array without data type");
    println!("\n array 2 is {:?}",arr2);
    println!(" The size of the array is {}",arr2.len());

    //Array with  default values of when stated
    let arr3:[i32;8] = [-1;8];
    println!("\n Array with default values");
    println!("\n array 3 is {:?}",arr3);
    println!(" The size of the array is {}",arr3.len());
}
