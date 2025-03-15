# 🎯 Guessing The Number Game

## 📝 Description

Welcome to Guessing The Number Game, a simple and fun command-line game built with Rust! The goal of the game is to guess a randomly generated number between 1 and 100. With each guess, the game provides feedback to help you get closer to the correct number.

## 🎮 How to Play

1. Run the game in your terminal.

2. The game will generate a random number between 1 and 100.

3. Enter your guess and press Enter.

4. The game will provide feedback:

  - Too small! 🔴 if your guess is lower than the target number.
  
  - Too big! 🔴 if your guess is higher than the target number.
  
  - BINGO, You Win! 🎉 if you guess the correct number.

5. The game will also give you hints if you're way off.

6. Keep guessing until you win! The game tracks the number of attempts.

7. After winning, you can choose to play again or exit. 

## 🔧 Requirements

Rust installed on your system. You can install it using Rustup.


## ▶️ Installing and Running the Game
   If you haven't already, clone the project from GitHub:
   ```sh
   git clone https://github.com/dossdortival/Rust-guessing-game.git
   
   cd Rust-guessing-game
   
   cargo build --release
   
   cargo run
   ``` 

# 📌 Features

- Random number generation between 1 and 100.

- Input validation to ensure only valid numbers are entered.

- Hint system to guide you when you're far off.

- Guess counter to track the number of attempts.

- Replay option after winning.

- Colorized output for better readability.

## 📜 License

This project is open-source and available under the MIT License.

Happy guessing! 🎲
