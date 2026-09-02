fn main(){
    let p1:f64 = 520_000_000.0;
    let r1:f64 = 10.0;
    let n1:f64 = 5.0;

    let a1= p1*(1.0+(r1/100.0)).powf(n1);
    println!("Amount is {}",a1);
    let ci1= a1-p1;
    println!("Compound Interest is {}",ci1);
    println!();


    println!("Compound Interest per year");
    println!();
    println!("Year 1");
    let n2:f64 = 1.0;
    
    let a2 = p1*(1.0+(r1/100.0)).powf(n2);
    let ci2= a2-p1;
    println!("Compoun Interest for Year 1 is {}",ci2);
    let p2= p1+ci2;
    println!(" New Principal is {}",p2);
    println!();
    println!("Year 2");
    let a3=p2*(1.0+(r1/100.0)).powf(n2);
    let ci3= a3-p2;
    println!("Compound Interest for Year 2 is {}",ci3);
    let p3= p2+ci3;
    println!(" New Principal is {}",p3);
    println!();
    println!("Year 3");
    let a4=p3*(1.0+(r1/100.0)).powf(n2);
    let ci4= a4-p3;
    println!(" Compound Interest for Year 3 is {}",ci4);
    let p4= p3+ci4;
    println!(" New Prinipal is {}",p4);
    println!();
    println!("Year 4");
    let a5=p4*(1.0+(r1/100.0)).powf(n2);
    let ci5=a5-p4;
    println!(" Compound Interest for Year 4 is {}",ci5);
    let p5= p4+ci5;
    println!(" New Prinipal is {}",p5);
    println!();
    println!("Year 5");
    let a6=p5*(1.0+(r1/100.0)).powf(n2);
    let ci6=a6-p5;
    println!(" Compound Interest for Year 5 is {}",ci6);
    println!();
    let b1=ci2+ci3+ci4+ci5+ci6;
    println!("Total Compound Interest across the 5 years is {}",b1);








}