use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("failed to delete");
    println!("File is removed");
}
