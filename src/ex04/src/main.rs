//use std::mem;

fn operators() {
    // arithmetic
    let mut a = 1 + 2 - 3 * 5 / 5;
    println!("a = {}", a);

    a += 11; // n tem ++ e --
            // += -= *= /= %=
    println!("a = {}", a);

    println!("mod {}/{} = {}", a, 3, (a%3));

    let a_cub = i32::pow(a, 3);
    println!("a³ = {}", a_cub);

    let b = 2.5;
    let b_cub = f64::powi(b, 3); // elevar a um int
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // elevar a um float
    println!("b³ = {}, b^pi = {}", b_cub, b_to_pi);

    // bitwise
    let c = 1 | 2;// | OR &, AND, ^ XOR, ! NOR
                  // 01 | 10 == 11 == 3_10
    println!("c = {}", c);

    let two_to_ten = 1 << 10; // >>
    println!("2^10 = {}", two_to_ten);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; 
    // ==, !=, >, <, >=, <=
    println!("pi < 4 = {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("x == 5 = {}", x_is_5);
}
/*
fn scope_and_shadowing() {
    let a = 123;
    let a = 1234;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    //println!("outside,  b = {}", b);
}
*/
fn main() {
    //scope_and_shadowing();
    operators();
}