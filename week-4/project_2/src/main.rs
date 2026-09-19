// A Rust Program to calculate worker's incentives

use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();


     // Now incase the user decides to be silly and types smth other than yes or no.Loop takes them back.
    // this loop is for a string that is why string is written
    let experience : String = loop {  
        experience.clear();     // this is to start afresh after the loop goes back

    println!("\nAre you experienced?");
    println!("\nyes / No");
    io::stdin()
    .read_line(&mut experience)
    .expect(" Not a valid string");
    
    let experience = experience.trim().to_lowercase() ;
       // Use trim to remove the \n when putting the condition ==(yes)
       // use lowercase to convert everything to lowercase in case they type uppercase
       if experience =="yes" { 
        break experience; //It stops the loop and saves that value as experience
    }
    if experience == "no" {
        break experience;  //It stops the loop and saves that value as experience
    }
    
    else {
        println!(" Type either yes or no!") // This is when they type anything else. it goes back to the top
    }; } ; 
 


    
    println!("Please enter your age");
    io::stdin()
    .read_line(&mut age)
    .expect("Not a valid string");

    let a:i8 = age
    .trim()
    .parse()
    .expect(" Not a valid age");

    
    
      
   
     if experience == "yes" {
        if a< 28 {
            let e_28:f32 = 1_300_000.0;
            println!(" Your monthly incentive is {} naira ",e_28);
        let a_e_28 = 12.0*e_28;
        println!();
        println!(" Your annual incentive is {} naira", a_e_28)
        }
        if a>=30 && a<=39{
            let e_30:f32 = 1_480_000.0;
            println!(" Your monthly incentive is {} naira ",e_30);
        let a_e_30 = 12.0*e_30;
        println!();
        println!(" Your annual incentive is {} naira", a_e_30)
        }
        if a>=40 {
            let e_40:f32 = 1_560_000.0;
            println!(" Your monthly incentive is {} naira",e_40);
        let a_e_40 = 12.0*e_40;
        println!();
        println!(" Your annual incentive is {} naira", a_e_40)
        }
         if a>=28 && a<=29  {
            println!(" So sorry, There is no incentive attached to this age ");
         }  
       
        // This else is for no
        // The else is used because i previously eliminated any other possible value of experience other than yes and no
    }else  { 
            let ne:f32 = 100_000.0;
    
        let ane = 12.0*ne ;
        println!();
        println!(" Your annual incentive is {} naira", ane);}
    
}
    


