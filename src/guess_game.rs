use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn readline() -> u32 {
    let mut guess = String::new();

    println!("Digite um numero: ");
    io::stdin()
            .read_line(&mut guess)
            .expect("Falha na leitura");

    return guess.trim().parse().expect("FAIL");
}

fn game() {
    let secret = rand::rng().random_range(1..=10);

    loop {
        let guess = readline();

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn main() {
    game();
}