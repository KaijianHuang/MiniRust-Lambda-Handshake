use std::io;
use rand::Rng;

fn main() {
    println!("Enter the start of the range:");
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read start number.");
    let start: u32 = start.trim().parse().expect("Please enter a valid number.");

    println!("Enter the end of the range:");
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("Failed to read end number.");
    let end: u32 = end.trim().parse().expect("Please enter a valid number.");

    let range = start..=end;
    let lucky_number = rand::thread_rng().gen_range(range);
    println!("Your lucky number is: {}", lucky_number);
}
