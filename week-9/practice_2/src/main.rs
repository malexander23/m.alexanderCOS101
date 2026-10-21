use std::io::Read;


fn main() {
    let mut file = std::fs::File::open("welcome_message.txt").expect("Open failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    println!("{}",contents);
}
