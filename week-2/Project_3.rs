fn main() {
let t:f64 = 210_000.0;
let r:f64 = 5.0;
let n:f64 = 3.0;

let a=t*(1.0 -(r/100.0)).powf(n);
println!("Cost price of the TV is{}",t);
println!();
println!( "Depreciation of the TV after 3 years is{}",a )
}