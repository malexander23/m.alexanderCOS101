fn main() {
    let mut mountain_heights = ("Everest",8848,"Fishtail",6993);
    println!("\nOriginal tuple = {:?}",mountain_heights);
    println!("\nThe first mountain is {} with the height of {} km", mountain_heights.0,mountain_heights.1);
    println!("\nThe second  mountain is {} with the height of {} km", mountain_heights.2,mountain_heights.3);
    
    
    mountain_heights.2 ="Lhotse";
    mountain_heights.3 = 8516;

    println!("\nChanged tuple = {:?}", mountain_heights);
    println!("\nThe first mountain is {} with the height of {} km", mountain_heights.0,mountain_heights.1);
    println!("\nThe second mountain is {} with the height of {} km", mountain_heights.2,mountain_heights.3);
}
