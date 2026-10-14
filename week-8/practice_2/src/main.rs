fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input = String::new();
    println!("Enter an index value between (0-7)");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let index:usize = input.trim().parse().expect("Invalid input");

    let ch: char= v[index];
    print!("{} is the character for index [{}]\n",ch,index);
}
