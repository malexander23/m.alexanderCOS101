fn value(n:Option<&char>){
    println!("Element of vector {:?}",n);
}




fn main() {
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    let mut input =String::new();
    println!("\nEnter an index value between (0-8)");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let index:usize = input.trim().parse().expect("invalid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
}
