use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;
use clap::Parser;

/// CLI tool for playing a number guessing game.
#[derive(Parser)]
#[command(name = "Guessing Game")]
#[command(version = "1.0")]
#[command(about = "Guess a randomly generated number!", long_about = None)]

struct Args {
    /// Minimum number in the range (default: 1)
    #[arg(default_value_t = 1)]
    min: i32,

    /// Maximum number in the range (default: 100)
    #[arg(default_value_t = 100)]
    max: i32,

    /// Disable colored output
    #[arg(long, default_value_t = false)]
    no_color: bool,
}

fn main() {
    let args = Args::parse();

    if args.min >= args.max {
        eprintln!("{}", "Error: Minimum value must be less than maximum!".red().bold());
        return;
    }

    println!("{}", "=== Guessing The Number GAME! ===".bold());

    let mut rng = rand::thread_rng();
    let mut secret_number = rng.gen_range(args.min..=args.max);
    let mut attempts = 0;

    loop {
        println!("Please input your guess number ({}-{}):", args.min, args.max);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let guess: i32 = match input.trim().parse() {
            Ok(num) if num >= args.min && num <= args.max => num,
            _ => {
                println!("{}", "Invalid number! Enter a valid number.".yellow().bold());
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small!".red().bold());
                if secret_number - guess <= 15 || guess - secret_number <= 15 {
                    println!("{}", "Hint: You are close! 🔥".yellow().bold());
                }
                else {
                    println!("{}", "Hint: You are far!".blue().bold());
                }
            }
           
            Ordering::Greater => {
                println!("{}", "Too big!".red().bold());
                if guess - secret_number <= 15 || secret_number - guess <= 15 {
                    println!("{}", "Hint: You are close! 🔥".yellow().bold());
                }
                else {
                    println!("{}", "Hint: You are far!".blue().bold());
                }
            }
            Ordering::Equal => {
                println!("{}", format!("BINGO, You Win! 🎉 It took you {} attempts.", attempts).green().bold());

                println!("{}", "Do you want to play again? (yes/y)".blue().bold());
                let mut play_again = String::new();
                io::stdin().read_line(&mut play_again).expect("Failed to read input");

                if !["y", "yes"].contains(&play_again.trim().to_lowercase().as_str()) {
                    println!("{}", "Thanks for playing! Goodbye!".bold());
                    break;
                }

                // Reset the game
                secret_number = rng.gen_range(args.min..=args.max);
                attempts = 0;
                println!("{}", "=== Guessing The Number GAME! ===".bold());
                continue;
            }
        }
    }
}

