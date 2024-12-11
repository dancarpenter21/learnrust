use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Target {secret_number}");

    loop {
        println!("Input your guess ...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("IO error reading user input");

        // note the 'match' call here that sends us down
        // the [Ok, Err] branches
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess");
                continue;
            }
        };

        println!("You guessed: {guess}");

        let _ = match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        };  // this ';' must be here if we use let x = match ...
    }

}
