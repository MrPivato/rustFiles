fn main() {
    let mut x = 5;
    println!("val de x = {}", x);
    x = 6;
    println!("val de x = {}", x);
    
    const MAX_POINTS: u32 = 100_000;
    println!("max points: {}", MAX_POINTS);
    
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("val de x: {}", x);
    
    let spaces = "     ";
    let spaces = spaces.len();
    //let mut spaces = "   ";
    //spaces = spaces.len();
    println!("spaces: {}", spaces);
}
