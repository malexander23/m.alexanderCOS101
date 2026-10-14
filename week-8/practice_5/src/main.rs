fn main() {
    
    let mut city: Vec<String> = Vec::new();
    println!("The City vector has element {}",city.len());
    let mut input = String::new();
    println!("\nList your favourite cities");
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let city_num:i32=input.trim().parse().expect("Inavlid input");
    for count in 0..city_num{
        let mut input_1 = String::new();
        println!("Enter City {}",count+1);
        std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
        let new_city:String = input_1.trim().parse().expect("inavlid input");
        city.push(new_city);
    }

    print!("\nYour preferred cities are:\n");
    let mut count = 1;
    for i in city{
        println!("{} {}",count,i);
        count+=1;
    }
}
