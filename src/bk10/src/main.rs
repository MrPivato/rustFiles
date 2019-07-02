fn main() {
    let mut s = String::from("hello");

    {
        let r2 = &mut s;
        println!("{},", r2);
    };

    //let r1 = &mut s;
    //println!("{},", r1);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);

    let len = calculate_len(&s);

    change(&mut s);

    println!("{} length = {}", s, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
