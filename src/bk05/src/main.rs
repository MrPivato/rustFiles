fn main() {
    let num = 3;

    if num > 5 {
        println!("maior que 5");
    } else {
        println!("menor que 5");
    }

    if num != 0 {
        println!("dif de 0!");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let num = if condition {
        5
    } else {
        6
    };

    println!("val num = {}", num);

    let mut c = 0;

    loop {
        c += 1;
        println!("{}", c);
        if c == 10 { break; }
    }

    c = 0;

    let result = loop {
        c += 1;
        if c == 10 {
            break c * 2;
        }
    };

    println!("result = {}", result);

    let mut n = 4;
    while n != 0 {
        println!("{}", n);
        n -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    println!("{:?}", a.iter());
    for element in a.iter() {
        println!("val = {}", element);
    }
    
    for n in (1..4).rev() {
        println!("{}", n);
    }
}
