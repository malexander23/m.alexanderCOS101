fn main() {
    let fullname = "Alexander Mkpoikanke Nsidibe";
    let department = "Software Engineering";
    let university = "Pan Atlantic University";

    let mut school = "School of Science".to_string(); // to convert it to a string object
    // You can only push to a string object
    school.push_str(" and Technology");

    println!("My name is : {}",fullname);
    println!("The lenght of my name fullname (including the space) is: {}",fullname.len());
    println!( "I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", university);
}
