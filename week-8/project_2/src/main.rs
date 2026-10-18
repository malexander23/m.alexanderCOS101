use std::io;
fn main() {
    println!("Welcome to the EY Interview Program\n");
    println!("EY Nigeria is scouting for the most experienced Developer \n");
    println!("Wishing you the best you the best\n");
    let mut  applicant:Vec<(String,u8)> = Vec::new();
let mut count=0;

let mut  n=String::new();
let mut a:u8 ;

    loop{
        println!("Applicant {}",count+1);
     loop{
        println!("Please enter your full name");
        let mut name_1 = String::new();
        io::stdin().read_line(&mut name_1).expect("Failed to read input");
         n = name_1.trim().to_string();
        if n ==""{
            println!("Error: You have not typed a name\n");
        }
        else{
        
            println!("Applicant's name : {}",n);
            break
        }
       
    };
    loop{
        println!("\nPlease enter your years of experience");
        println!("enter a whole number");
    
        let mut ex = String::new();
        io::stdin().read_line(&mut ex).expect("Failed too read input");
        a=ex.trim().parse().expect("Invalid input");
       
            println!("\nApplicant's years of experience : {}",a);
            break
        
    }
    applicant.push((n.clone(),a));
    count+=1;
    println!("Do you want to enter another applicant's details");
    
    println!("Enter Yes or No");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read input");
    let ans= answer.trim().to_lowercase();
    if ans =="yes"{
        println!("\nLoading... \n");
        
    }
    if ans =="no"{
        break
    }
    

};

    

    let mut winner =&applicant[0]; // winner is the tuple as whole. apllicant{0} is the first tuple
    for app in &applicant{    // to check through the whole vector
if app.1> winner.1{
    winner=app;
}
    }

    println!("\nThe interview has been conculuded\n");
    println!("\n This is a list of all the applicants\n");
     
    for (n,a) in &applicant{
        println!("\nName:{}  and experience {} years",n,a);
    }

    println!("\nThe most experienced applicant is ...\n");
    println!("Name:{}\n Years of experience :{}",winner.0,winner.1);

}

