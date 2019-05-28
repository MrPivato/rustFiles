use std::mem;

fn main() {
    // i8 u8 i16 u16 i32 u32 i64 u64
    // unsigned
    let x:u8 = 200; // 0..255
    // signed
    let _y:i8 = -127; // -127..128

    println!("x = {}, y = {}", x, _y);

    let mut a:u8 = 10; // mutable
    a += 1;
    println!("a = {}", a);

    let mut b = 123456789; // 32 bit signed i32
    println!("b = {}, size = {} bytes, {}", b, mem::size_of_val(&b), &b);
    b = -1;
    println!("b = {}", b);

    let z:isize = 123; // isize usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}bit OS", 
    z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes, {}", d, mem::size_of_val(&d), &d);

    let e:f64 = 2.5; // double-precision, 8 bytes ou 64 bits, f64
    println!("e = {}, size = {} bytes, {}", e, mem::size_of_val(&e), &e);

    let condition = true;
    println!("condition = {}, size = {} bytes, {}", 
    condition, mem::size_of_val(&condition), &condition);

    if condition {
        println!("Muito brabo!");
    } else {
        println!("Pouco brabo!")
    }
}
