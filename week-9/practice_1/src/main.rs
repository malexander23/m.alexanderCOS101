use std::io::Write;
fn main(){
    let announce = "Week 9 - Rust File Input & Output\n";
    let department = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("failed to write ");
    file.write_all(announce.as_bytes()).expect("Failed to write");
    file.write_all(department.as_bytes()).expect("Failed to write");
    println!("\nData written to file");
}
