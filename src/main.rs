use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::RangeInclusive;
use std::process::exit;

// declare helper constants
const GUESS_RANGE: RangeInclusive<u8> = 1..=100;
const MAXIMUM_GUESSES: u8 = 7;

fn main() {
    println!(
        "Guess the number between {} and {}!",
        GUESS_RANGE.start(),
        GUESS_RANGE.end()
    );

    let secret_number = rand::thread_rng().gen_range(GUESS_RANGE);

    let mut current_guess = 1;

    loop {
        // check if we can still guess
        if current_guess > MAXIMUM_GUESSES {
            println!("You lose!");
            println!("Answer was {secret_number}");
            break;
        }

        println!("------------- Guess {current_guess} of {MAXIMUM_GUESSES} ---------------");
        println!("Please input your guess.");

        // declare variable to that will receive user input
        let mut guess = String::new();

        // fetch users' input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim line break
        guess = guess.trim().to_string();

        // check whether user doesn't want to end the game prematurely
        if guess == "quit" || guess == "exit" {
            println!("Exiting game");
            exit(0);
        }

        // check whether guess is within valid guess range
        let guess: u8 = match guess.parse() {
            Ok(num) if GUESS_RANGE.contains(&num) => num,
            _ => {
                println!(
                    "Input \"{guess}\" is not within allowed range {} - {}",
                    GUESS_RANGE.start(),
                    GUESS_RANGE.end()
                );
                continue;
            }
        };

        println!("You guessed: {guess}");

        // check users' guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        current_guess += 1;
    }
}
