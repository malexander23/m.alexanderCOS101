use std::io;
fn main() {
    println!("Hello, User");
    println!("This is a Public Service APS level checker");
    println!("\nPlease enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("\nHello,{}",name);

    println!("Please answer the following questions");
    println!("From the following sections, which do you fall under");
    let s_1=vec!["Office Admin","Academic","Lawyer","Teacher"];

    for i in 0..s_1.len(){
    println!("{}",s_1[i]);
    }
    println!();
    let mut section = String::new();
    io::stdin().read_line(&mut section).expect("Failed to read input");
    let s = section.trim().to_lowercase();
    println!("You are a/an {}",s);
    println!();
    
    if s == "office admin"{
        admin();

    }
    if s == "academic"{
        acad();
    }
    if s =="lawyer"{
        law();
    }
    if s=="teacher"{
        teach();
    }



}
fn admin(){
    let a = vec!["Intern", "Administrator","Senior Administrator", "Office Manager", "Director","CEO"];
    println!("Choose from these options");
    println!(" Please type your answer exactly how you see it below \n");
    for i in 0..a.len(){
        println!(" {}",a[i]);
        
     }
println!();
    
       let  mut code = String::new();
       io::stdin().read_line(&mut code).expect("Failed to read input");
       let c = code.trim().to_lowercase();

       println!("\nYou are almost there");
       println!(" \nPlease could you enter your years of experience");
       println!("Use a whole number. eg 4, 5... etc\n not 4.5 or four and the half ");

       let mut ex = String::new();
       io::stdin().read_line(&mut ex).expect("Failed to read input");
       let e:i8 = ex.trim().parse().expect("Invalid input");

        
       if c =="intern"||e==1||e==2{
        println!("\nYour APS LEVEL is APS 1-2");
        println!("Congratulations");

       }
       else if c =="administrator" || (e>=3||e<=4){
        println!("\nYour APS LEVEL is APS 3-5");
        println!("Congratulations");

       }

       else if c =="senior administrator" || (e>=5||e<=7){
        println!("\nYour APS LEVEL is APS 5-8");
        println!("Congratulations");

       }
      else  if c =="office manager" || (e>=8||e<=9){
        println!("\nYour APS LEVEL is EL1 8-10");
        println!("Congratulations");

       }
       else if c =="director" || (e>=10||e<=13){
        println!("\nYour APS LEVEL is EL2 10-13");
        println!("Congratulations");

       }
       else if c =="ceo" {
        println!("\nYour APS LEVEL is SES");
        println!("Congratulations");

       }

}
fn acad(){
    let a_1 = vec!["Research Assistant", "PhD Candidate","Post-Doc Researcher", "Senior Lecturer", "Dean"];
    println!("Choose from these options");
    println!(" Please type your answer exactly how you see it below \n");
    for i in 0..a_1.len(){
        println!(" {}",a_1[i]);
        
     }
println!();
    
       let  mut code_1 = String::new();
       io::stdin().read_line(&mut code_1).expect("Failed to read input");
       let c_1 = code_1.trim().to_lowercase();

       println!("\nYou are almost there");
       println!(" \nPlease could you enter your years of experience");
       println!("Use a whole number. eg 4, 5... etc\n not 4.5 or four and the half ");

       let mut ex_1 = String::new();
       io::stdin().read_line(&mut ex_1).expect("Failed to read input");
       let e_1:i8 = ex_1.trim().parse().expect("Invalid input");

        
       if c_1 =="research assistant" || (e_1>=3||e_1<=4){
        println!("\nYour APS LEVEL is APS 3-5");
        println!("Congratulations");

       }

       else if c_1 =="phd candidate" || (e_1>=5||e_1<=7){
        println!("\nYour APS LEVEL is APS 5-8");
        println!("Congratulations");

       }
       else if c_1 =="post-doc researcher" || (e_1>=8||e_1<=9){
        println!("\nYour APS LEVEL is EL1 8-10");
        println!("Congratulations");

       }
       else if c_1 =="senior lecturer" || (e_1>=10||e_1<=13){
        println!("\nYour APS LEVEL is EL2 10-13");
        println!("Congratulations");

       }
       else if c_1 =="dean" {
        println!("\nYour APS LEVEL is SES");
        println!("Congratulations");

       }

}
fn law(){
    let a_2 = vec!["Paralegal", "Junior Associate","Associate", "Senior Associate 1-2", "Senior Associate 3-4","Partner"];
    println!("Choose from these options");
    println!(" Please type your answer exactly how you see it below \n");
    for i in 0..a_2.len(){
        println!(" {}",a_2[i]);
        
     }
println!();
    
       let  mut code_2 = String::new();
       io::stdin().read_line(&mut code_2).expect("Failed to read input");
       let c_2 = code_2.trim().to_lowercase();

       println!("\nYou are almost there");
       println!(" \nPlease could you enter your years of experience");
       println!("Use a whole number. eg 4, 5... etc\n not 4.5 or four and the half ");

       let mut ex_2 = String::new();
       io::stdin().read_line(&mut ex_2).expect("Failed to read input");
       let e_2:i8 = ex_2.trim().parse().expect("Invalid input");

        
       if c_2 =="paralegal"||e_2==1||e_2==2{
        println!("\nYour APS LEVEL is APS 1-2");
        println!("Congratulations");

       }
        else if c_2 =="junior associate" || (e_2>=3||e_2<=4){
        println!("\nYour APS LEVEL is APS 3-5");
        println!("Congratulations");

       }

       else if c_2 =="associate" || (e_2>=5||e_2<=7){
        println!("\nYour APS LEVEL is APS 5-8");
        println!("Congratulations");

       }
       else if c_2 =="senior associate 1-2" || (e_2>=8||e_2<=9){     //check if it is 1 to 2 years of experience
        println!("\nYour APS LEVEL is EL1 8-10");
        println!("Congratulations");

       }
       else if c_2 =="senior associate 3-4" || (e_2>=10||e_2<=13){
        println!("\nYour APS LEVEL is EL2 10-13");
        println!("Congratulations");

       }
       else if c_2 =="partner" {
        println!("\nYour APS LEVEL is SES");
        println!("Congratulations");

       }

}
fn teach(){
     let a_3 = vec!["Placement", "Classroom Teacher","Senior Teacher", "Leading Teacher", "Deputy Principal","Principal"];
    println!("Choose from these options");
    println!(" Please type your answer exactly how you see it below \n");
    for i in 0..a_3.len(){
        println!(" {}",a_3[i]);
        
     }
println!();
    
       let  mut code_3 = String::new();
       io::stdin().read_line(&mut code_3).expect("Failed to read input");
       let c_3 = code_3.trim().to_lowercase();

       println!("\nYou are almost there");
       println!(" \nPlease could you enter your years of experience");
       println!("Use a whole number. eg 4, 5... etc\n not 4.5 or four and the half ");

       let mut ex_3 = String::new();
       io::stdin().read_line(&mut ex_3).expect("Failed to read input");
       let e_3:i8 = ex_3.trim().parse().expect("Invalid input");

        
       if c_3 =="placement"||e_3==1||e_3==2{
        println!("\nYour APS LEVEL is APS 1-2");
        println!("Congratulations");

       }
       else if c_3 =="classroom teacher" || (e_3>=3||e_3<=4){
        println!("\nYour APS LEVEL is APS 3-5");
        println!("Congratulations");

       }

       else if c_3 =="senior teacher" || (e_3>=5||e_3<=7){
        println!("\nYour APS LEVEL is APS 5-8");
        println!("Congratulations");

       }
      else if c_3 =="leading teacher" || (e_3>=8||e_3<=9){     //check if it is 1 to 2 years of experience
        println!("\nYour APS LEVEL is EL1 8-10");
        println!("Congratulations");

       }
       else if c_3 =="deputy principal" || (e_3>=10||e_3<=13){
        println!("\nYour APS LEVEL is EL2 10-13");
        println!("Congratulations");

       }
       else if c_3 =="principal" {
        println!("\nYour APS LEVEL is SES");
        println!("Congratulations");

       }

}
