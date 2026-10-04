fn main() {
    let a: i32 =10;
    let b:i32 = 20;

    println!(" Value of A : {}", a);
    println!(" Value of B : {}", b);
    
    let mut result = a>b;
    println!(" \nA  is greater than  B : {}", result);

    result = a<b;
    println!(" A is lesser than B : {}", result);

    result = a>= b;
    println!(" A is greater than or equal to B : {}", result);

    result = a<= b;
    println!(" A is lesser than or equal to B : {}", result);

    result = a==b;
    println!(" A is equal to B : {}", result);

    result = a!=b;
    println!(" A is not equal to B : {}", result);





}
