// Title: Number Guessing Game
// Description: A simple number guessing game where a computer will generate a random number between 1 and 100 and the user will try to guess it.
// if the user guesses the number correctly, the game will end and the user will be notified.
// if the user guesses the number incorrectly, the game will continue and the user will be notified if the number is higher or lower than the guess.
// Difficulty: 1
// Tags: random, guessing, game, number

use rand::Rng;
use std::io::stdin;

pub fn number_guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");
        let mut buffer = String::new();
        // stdin().read_line(&mut buffer).expect("Failed to read line");
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim().parse::<u32>();
                match parsed {
                    Ok(guess) => {
                        println!("You guessed: {}", guess);
                        if guess < 1 || guess > 100 {
                            println!("Please type a number between 1 and 100!");
                            continue;
                        }

                        match guess.cmp(&secret_number) {
                            std::cmp::Ordering::Less => println!("Too small!"),
                            std::cmp::Ordering::Greater => println!("Too big!"),
                            std::cmp::Ordering::Equal => {
                                println!("You win!");
                                break;
                            }
                        }
                    }
                    Err(e) => println!("Please type a number!, {}", e),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
