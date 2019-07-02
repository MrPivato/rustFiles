fn main() {
    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    let mut s = String::from("hello");
    s.push_str(", world!");// push_str() appends a literal to a String

    println!("{}", s);

    let x = 5; //pushed to the stack
    let _y = x; //pushed to the stack

    let s1 = String::from("hello"); // allocated on the heap
    //let s2 = s1; // s1 is invalidated
    let s2 = s1.clone(); // allocated on the heap

    println!("s1 = {}, s2 = {}", s1, s2);
}
