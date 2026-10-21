use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Failed to open file ");
    file.write_all("\nHello Class".as_bytes()).expect("Failed to write");
    file.write_all("\nThis is the appendage to the document data txt".as_bytes()).expect("Failed to write");
    println!("\nfile append success");
  
    use std::io::Read;
    let mut lex = std::fs::File::open("data.txt").expect("Failed to read");
    let mut inside= String::new();
    lex.read_to_string(&mut inside).expect("failed to read");
    print!("{}\n",inside);
}
