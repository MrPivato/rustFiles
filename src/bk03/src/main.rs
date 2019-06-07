fn main() {
    //let guess = "42".parse().expect("N.A.N!");
    //println!("{}", guess);
    
    // ------- Scalar types -------
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    let t = true;
    let f: bool = false; // explicit type annotation
    
    let c = 'z';
    let z = 'ℤ';
    let cat = '😻';
    
    println!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}", 
        x, y, sum, difference, product, quotient, remainder,
        t, f, c, z, cat);
    
    // ------- Compound types -------
    let tup: (i32, f64, u8) = (500, 6.4, 1); // usar {:?} ou {:#?} para printar
    let (a, b, c) = tup; // destruct, quebra em 3 vars
    println!("{:?}|{}|{}|{}", tup, a, b, c);
    
    let val0 = tup.0;
    let val1 = tup.1;
    let val2 = tup.2;
    println!("{}|{}|{}", val0, val1, val2);
    
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{:#?}", months);
    /*
    let arr2 = [3; 5]; // vai ter 5 num 3
    let first = arr2[0];
    let second = arr2[2];
    println!("{}", arr2[10]);
    */
}
/*
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
(pode botar um sufixo como 57u8)
*/