fn main() {
    another_function(10, 1000);
    
    let x = 11;
    
    let y = {
        let x = 3;
        x
    };
    
    println!("val y = {}", y);
    println!("val x = {}", x);
    
    let x = five();
    println!("x = {}", x);
    
    let x = plus_one(7);
    println!("x = {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("uma func massa {}:{}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1   
}