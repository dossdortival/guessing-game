use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

// Define a Guess struct to validate input
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("Guess value must be between 1 and 100, got {}.", value));
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() {
    println!("{}", "===Guessing The Number GAME!===".bold());
    
    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0; // Record the number of attempts
    
    loop {
        attempts += 1;
        println!("Please input your guess number.");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Use the Guess struct to validate input
        let guess = match input.trim().parse::<i32>() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess,
                Err(err_msg) => {
                    println!("{}", err_msg.yellow().bold());
                    continue;
                }
            },
            Err(_) => {
                println!("{}", "Please enter a valid number!".yellow().bold());
                continue;
            }
        };
        
        println!("You guessed: {}", guess.value());
        
        match guess.value().cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small!".red().bold());
                if secret_number - guess.value() > 20 {
                    println!("{}", "Hint: You're way too low!".cyan());
                }
            }
            Ordering::Greater => {
                println!("{}", "Too big!".red().bold());
                if guess.value() - secret_number > 20 {
                    println!("{}", "Hint: You're way too high!".cyan());
                }
            }
            Ordering::Equal => {
                println!("{}", format!("BINGO, You Win! 🎉 It took you {} attempts.", attempts).green().bold());

                // Ask if the player wants to play again
                println!("{}", "Do you want to play again? (y/n)".blue().bold());

                let mut play_again = String::new();
                io::stdin().read_line(&mut play_again).expect("Failed to read input");

                if !["y", "yes"].contains(&play_again.trim().to_lowercase().as_str()){
                    println!("{}", "Thanks for playing! Goodbye!".bold());
                    break;
                } else {
                    // Reset the game
                    secret_number = rand::thread_rng().gen_range(1..=100);
                    attempts = 0;
                    println!("{}", "===Guessing The Number GAME!===".bold());
                    continue;
                }
            }
            _ => {} // Other cases ("Too small!" and "Too big!") remain unchanged.
        }
    }
}

