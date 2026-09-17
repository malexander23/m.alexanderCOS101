// Rust code for making a continuous loop of counting till number 15

fn main() {
    let mut x = 0;
    loop{
        x+=1;
        println!("x = {}",x);

        if x==15 {
            break;
        }
    }
    

}
