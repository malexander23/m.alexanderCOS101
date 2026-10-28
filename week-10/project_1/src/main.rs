use std::io ;

struct Laptopbrand{
    name:String,
    price:u32,
    quantity:u32
}



fn main() {
    println!("Welcome to Alaba Electronics\n");

    println!("This is a list of the available laptops for sale\n");
     let m=vec!["HP laptop","IBM Laptop", "Toshiba Laptop","Dell Laptop"];
     let up= vec![650_000,755_000,550_000,850_000];
     let n = vec![10,6,10,4];

     for i in 0..m.len(){
    println!("{:?} for the unit price of {:?} Naira\n There are {:?} available for sale ",m[i],up[i],n[i]);
}

//HP
let mut q1:u32 = 0;

loop{
println!("\nHow many HP laptops do you want to buy\n");

let mut q =String::new();

        io::stdin().read_line(&mut q).expect("Failed to read input");
        q1 = q.trim().parse().expect("Invalid input");
    println!("You want {} HP laptops\n",q1);

    println!("Is your order correct\n Yes or No");
    let mut answer =String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read input");
        let mut ans = answer.trim().to_lowercase();
        if ans =="yes"{
            break
        }
}

//IBM
let mut iq1:u32=0;

loop{
println!("How many IBM laptops do you want to buy\n");

let mut iq =String::new();

        io::stdin().read_line(&mut iq).expect("Failed to read input");
        iq1= iq.trim().parse().expect("Invalid input");
    println!("You want {} IBM laptops\n",iq1);

    println!("Is your order correct\n Yes or No");
    let mut answer1 =String::new();
        io::stdin().read_line(&mut answer1).expect("Failed to read input");
        let mut ans1 = answer1.trim().to_lowercase();
        if ans1 =="yes"{
            break
        };
}
//Toshiba
let mut tq1:u32=0;

loop{
println!("How many Toshiba laptops do you want to buy\n");

let mut tq =String::new();

        io::stdin().read_line(&mut tq).expect("Failed to read input");
        tq1= tq.trim().parse().expect("Invalid input");
    println!("You want {} IBM laptops\n",tq1);

    println!("Is your order correct\n Yes or No");
    let mut answer2 =String::new();
        io::stdin().read_line(&mut answer2).expect("Failed to read input");
        let mut ans2 = answer2.trim().to_lowercase();
        if ans2 =="yes"{
            break
        };
}
//Dell
let mut dq1:u32=0;

loop{
println!("How many Dell laptops do you want to buy\n");

let mut dq =String::new();

        io::stdin().read_line(&mut dq).expect("Failed to read input");
        dq1= dq.trim().parse().expect("Invalid input");
    println!("You want {} Dell laptops\n",dq1);

    println!("Is your order correct\n Yes or No");
    let mut answer3 =String::new();
        io::stdin().read_line(&mut answer3).expect("Failed to read input");
        let mut ans3 = answer3.trim().to_lowercase();
        if ans3 =="yes"{
            break
        };
}





    let HP = Laptopbrand{
        name:String::from("HP laptop"),
        price:650_000,
        quantity:q1


    };
    let IBM = Laptopbrand{
        name:String::from("IBM laptop"),
        price:755_000,
        quantity:iq1


    };
let Toshiba = Laptopbrand{
        name:String::from("Toshiba laptop"),
        price:550_000,
        quantity:tq1


    };
let Dell = Laptopbrand{
        name:String::from("Dell laptop"),
        price:850_000,
        quantity:dq1


    };
    let mut tot:Vec<u32> = Vec::new();
    println!("Your receipt\n");
    show(HP);
    show(IBM);
    show(Toshiba);
    show(Dell); 

let up1= vec![650_000,755_000,550_000,850_000];
    let s =(q1*up1[0])+ (iq1*up1[1])+ (tq1*up1[2])+(dq1*up1[3]);
    println!("Grand total = {} Naira",s);


}
fn show( a:Laptopbrand){
    let p=vec!["HP laptop","IBM Laptop", "Toshiba Laptop","Dell Laptop"];
    let t = a.price*a.quantity;
    
    
    println!("You have requested for {} of your desired laptop.\n Your total is {} Naira",a.quantity,t);
}


