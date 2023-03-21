
use std::io;

fn main() {
    // Prompt the user to enter a range
    println!("Enter a range of numbers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input range
    let range: Vec<&str> = input.trim().split('-').collect();
    let start = range[0].parse::<i32>().unwrap();
    let end = range[1].parse::<i32>().unwrap();

    // Generate a random lucky number within the range
    let lucky_number = rand::thread_rng().gen_range(start, end);
    
    // Print the lucky number
    println!("Your lucky number is {}", lucky_number);
}
