use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("Please input your guess: ");
        let mut guess = String::new();
         stdin.read_line(&mut guess).expect("Failed to read line");

        // parse to number
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
