use std::io;
fn main() {
    println!("Hello, Customer");
    println!(" \nWelcome to Kiki's Foods");
    println!("\n Would you like to order anything?");

    
    let mut answer = String::new();
    println!("\nyes / No");
    io::stdin()
    .read_line(&mut answer)
    .expect(" Not a valid string");
    
    let answer = answer.trim().to_lowercase() ;
    if answer =="yes" {
        let mut total = 0;

        loop{        
            
            println!( "\nThis is the list of foods available" );
            println!( "\n To order, please type the code");

            let p = " Poundo Yam/Edinkaiko Soup";
            let pa:u16= 3_200;

            let f = "Fried Rice & Chicken";
            let fa:u16 = 3_000;

            let a = "Amala & Ewedu Soup";
            let aa:u16 = 2_500;

            let e = "Eba & Egusi Soup";
            let ea:u16 = 2_000;

            let w ="White Rice & Stew";
            let wa:u16 = 2_500;

            println!(" 1.  {}.      The code is P.     Price: {} naira ",p,pa);
            println!(" 2.  {}.      The code is F.     Price: {} naira ",f,fa);
            println!(" 3.  {}.      The code is A.     Price: {} naira ",a,aa);
            println!(" 4.  {}.      The code is E.     Price: {} naira ",e,ea);
            println!(" 5.  {}.      The code is W.     Price: {} naira ",w,wa);

            println!(" Enter the code");
            let mut order_1=String::new();
            io::stdin().read_line(&mut order_1).expect("Not a valid string"); 

            if order_1.trim().to_uppercase() == "P"{
                println!(" How many portions do you want");

                let mut number= String::new();
                io::stdin().read_line(&mut number).expect("Not a valid string");
                let n_1:u16 = number.trim().parse().expect("Not a valid integer");
                total += pa * n_1;
                println!(" \nYou have ordered {} of {}",p,n_1);
                
                

            }
            else if order_1.trim().to_uppercase()  == "F"{
                println!(" How many portions do you want");

                let mut number= String::new();
                io::stdin().read_line(&mut number).expect("Not a valid string");
                let n_1:u16 = number.trim().parse().expect("Not a valid integer");
                total+= fa * n_1;
                println!(" \nYou have ordered {} of {}",f,n_1);
                

            
            }
            else if order_1.trim().to_uppercase()  == "A"{
                println!(" How many portions do you want");

                let mut number= String::new();
                io::stdin().read_line(&mut number).expect("Not a valid string");
                let n_1:u16 = number.trim().parse().expect("Not a valid integer");
                total+= aa * n_1;
                println!(" \nYou have ordered {} of {}",a,n_1);
                

            
            }else if order_1.trim().to_uppercase()  == "E"{
                println!(" How many portions do you want");

                let mut number= String::new();
                io::stdin().read_line(&mut number).expect("Not a valid string");
                let n_1:u16 = number.trim().parse().expect("Not a valid integer");
                total+= ea * n_1;
                println!(" \nYou have ordered {} of {}",e,n_1);
                

            
            }else if order_1.trim().to_uppercase()  == "W"{
                println!(" How many portions do you want");

                let mut number= String::new();
                io::stdin().read_line(&mut number).expect("Not a valid string");
                let n_1:u16 = number.trim().parse().expect("Not a valid integer");
                total+= wa * n_1;
                println!(" \nYou have ordered {} of {}",w,n_1);
                
                 
            
            }
            println!("\n Do you want to check out?Yes/No");
            let mut restart= String::new();
            io::stdin().read_line(&mut restart).expect("Not a valid string");

            if restart.trim().to_lowercase() == "no"{
                continue
            }else{
                break
            }
        }
              
        if total>10_000{
            println!("\n You have received a 5% discount on your purchase");
        total=total*(95/100);
            println!("Final Total:{} Naria", total);
        }else{
            println!("Final Total:{} Naria", total);
        }
    }
    else if answer =="no"{

        println!(" Thanks for coming");
        
    }
    else{
        println!(" Type either yes or no!")
    };



        


    }

    

//}
