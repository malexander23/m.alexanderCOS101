fn main() {
    let name_1 = "Alexander Mkpoikanke Nsidibe";
    println!(" My name is {}", name_1);

    // to replace an exisiting part
    println!("\nMy sister is beside me" );
    let name_2 = name_1.replace("Mkpoikanke","Edidiong"); // The first one is what is to be found. The second is the new input
    println!( "\nHer name is {}",name_2);

    // to remame the facult
    let faculty = "Faculty of Science and Technology";
    let school = faculty.replace("Faculty","School");
    println!(" \nWe are both students of {}", school);
}
