fn main(){
    println!("Sum of sales");
    println!();
    println!("Items sold");
    let t:f64 = 450_000.0;
    let q1:f64 = 2.0;
    let tt = t*q1;
    println!("2 Toshiba Laptops for a total of {}",tt);
    let m:f64 = 1_500_000.0;
    let q2:f64 = 1.0;
    let mt = m*q2;
    println!("1 Mac Laptop for a total of {}",mt);
    let h:f64 = 750_000.0;
    let q3:f64 = 3.0;
    let ht = h*q3;
    println!("3 HP Laptops for a total of {}",ht);
    let d:f64 = 2_850_000.0;
    let q4:f64 = 3.0;
    let dt = d*q4;
    println!("3 Dell Laptops for a total of {}",dt);
    let a:f64 = 250_000.0;
    let q5:f64 = 1.0;
    let at = a*q5;
    println!("1 Acer Laptop for a total of {}",at);
    let s= tt+mt+ht+dt+at;
    println!();
    println!("Sum of Sales Record is{}",s);
    println!();
    let it= q1+q2+q3+q4+q5;
    println!("Total Items sold is {}",it);
    let av= s/it;
    println!();
    println!("Average sales is{}",av);



}