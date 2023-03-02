use rand::Rng;
use std::io;

fn main() {
    println!("Let's play Rock-Paper-Scissors!");
    loop {
        println!("Enter your choice (rock, paper, or scissors):");
        let mut player_choice = String::new();
        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        let player_choice = player_choice.trim();
        if player_choice != "rock" && player_choice != "paper" && player_choice != "scissors" {
            println!("Invalid input. Please try again.");
            continue;
        }

        let computer_choice = get_computer_choice();
        println!(
            "You chose {} and the computer chose {}.",
            player_choice, computer_choice
        );

        let result = get_game_result(player_choice, computer_choice);
        match result {
            GameResult::Win => println!("You win!"),
            GameResult::Lose => println!("You lose!"),
            GameResult::Draw => println!("It's a draw."),
        }

        println!("Do you want to play again? (yes or no)");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
        if play_again.trim() != "yes" {
            break;
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_index = rand::thread_rng().gen_range(0..3);
    choices[random_index].to_string()
}

fn get_game_result(player_choice: &str, computer_choice: String) -> GameResult {
    if player_choice == "rock" && computer_choice == "scissors"
        || player_choice == "paper" && computer_choice == "rock"
        || player_choice == "scissors" && computer_choice == "paper"
    {
        GameResult::Win
    } else if player_choice == "rock" && computer_choice == "paper"
        || player_choice == "paper" && computer_choice == "scissors"
        || player_choice == "scissors" && computer_choice == "rock"
    {
        GameResult::Lose
    } else {
        GameResult::Draw
    }
}
