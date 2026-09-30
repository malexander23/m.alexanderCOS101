fn main() {
    let empty_string =String::new();
    println!("Lenght of the string is {}",empty_string.len()); // this is to count the number of characteres in the string

    let content_string = String::from("Computer Science");  // The space in between is counted
    println!("Lenght of the string with content is {}", content_string.len());

}
