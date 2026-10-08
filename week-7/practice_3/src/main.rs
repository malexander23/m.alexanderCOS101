fn value_pi()->f64{
    let a:f64 = 22.0;
    let b:f64 = 7.0;
    let c:f64 = a/b;
    c  // it is the same this as return c;  it makes a copy of c into value_pi
}




fn main() {
    println!("The value of pi is {}",value_pi());
}
