use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::rng().random_range(1..=50);

    println!("Guess a random number between 1 and 50!");
    print!("Please input your guess: ");
    io::stdout().flush().expect("io failed");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {guess}....which is ");
        io::stdout().flush().expect("io failed");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("too smol\nguess again... "),
            Ordering::Greater => print!("too large\nguess again... "),
            Ordering::Equal => {
                println!("a match!");
                break;
            }
        }
        io::stdout().flush().expect("io failed");
    }
}
