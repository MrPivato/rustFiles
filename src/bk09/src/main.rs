use std::io;

fn main() {
    let mut s1 = String::new();

    io::stdin().read_line(&mut s1)
                .expect("Não consegui ler");

    assert_eq!(s1.pop(), Some('\n'));

    let (s1, length) = s1_length(s1);

    println!("{} length = {}", s1, length);
}

fn s1_length(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}