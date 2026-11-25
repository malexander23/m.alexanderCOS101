use std::io;
use std::io::Read;
fn main() {
    println!("Hello User");
    println!("Welcome to Globacom");
    println!("Please are you a/an \nAdministrator, Code = A \nProject Manager, Code = P \nEmployee, Code = E \nCustomer, Code = C \nVendor, Code = V");
    println!("Type the code ");
 
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read input");
    let ans = answer.trim().to_lowercase();
     
    if ans == "a" {
        println!("Administrator Access granted\n");
        let mut file_1 = std::fs::File::open("globacom.sql").expect("Failed to open file");
        let mut a_content = String::new();
        file_1.read_to_string(&mut a_content).expect("Failed to read all");
        println!("{}",a_content);

    }
    if ans == "p"{
        println!("Project Manager Access granted\n");
        let mut file_2 = std::fs::File::open("project_tb.sql").expect("Failed to open file");
        let mut p_content = String::new();
        file_2.read_to_string(&mut p_content).expect("Failed to read all");
        println!("{}",p_content);

    }
    if ans == "e"{
        println!("Employee Access granted\n");
        let mut file_3 = std::fs::File::open("staff_tb.sql").expect("Failed to open file");
        let mut e_content = String::new();
        file_3.read_to_string(&mut e_content).expect("Failed to read all");
        println!("{}",e_content);

    }
    if ans == "c"{
        println!("Customer Access granted\n");
        let mut file_4 = std::fs::File::open("customer_tb.sql").expect("Failed to open file");
        let mut c_content = String::new();
        file_4.read_to_string(&mut c_content).expect("Failed to read all");
        println!("{}",c_content);

    }
    if ans == "v"{
        println!("Vector Access granted\n");
        let mut file_5 = std::fs::File::open("dataplan_tb.sql").expect("Failed to open file");
        let mut v_content = String::new();
        file_5.read_to_string(&mut v_content).expect("Failed to read all");
        println!("{}",v_content);

    }

}
