use std::io;
struct Employee{
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee{
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    
    let emp2 = Employee{
        company:String::from("Googke Inc"),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    display(emp1);
    display(emp2);
}
fn display(m:Employee){
    println!("Name is :{} company is {} age is {}",m.ceo,m.company,m.age);
}