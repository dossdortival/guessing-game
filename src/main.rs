use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            println!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("{}", "===Guessing The Number GAME!===".bold());
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess number.");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Use the Guess struct to validate input
        let guess = match input.trim().parse() {
            Ok(num) => match Guess::new(num) {
                guess @ _ => guess,
            },
            Err(_) => {
                println!("{}", "Please enter a valid number!".yellow().bold());
                continue;
            },
        };
        
        println!("You guessed: {}", guess.value());
        
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red().bold()),
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            Ordering::Equal => {
                println!("{}", "BINGO, You Win!".green().bold());
                break;
            }
        }
    }
}

