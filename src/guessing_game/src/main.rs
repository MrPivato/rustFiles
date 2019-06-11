use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand_generator();
    //println!("Adivinhe o num {}", secret_number);
    
    loop {
        let mut guess = String::new();

        println!("Digite seu palpite:");

        io::stdin().read_line(&mut guess)
            .expect("Não consegui ler");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Baixo!"),
            Ordering::Greater => println!("Alto!"),
            Ordering::Equal => {
                println!("Acertou!!");
                break;
            }
        }
    }
}

fn rand_generator() -> u32 {
    rand::thread_rng().gen_range(1, 101) //[1, 101)
}

// let guess: u32 = guess_validate(guess);
// fn guess_validate(guess: String) -> u32 {
//     guess.trim().parse()
//         .expect("Digite um NÚMERO!")
// }
