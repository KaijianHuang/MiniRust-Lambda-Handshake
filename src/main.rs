use std::io;

fn main() {
    println!("Enter the name of a popular city(new york city, london, paris, tokyo):");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read input.");

    match city.trim().to_lowercase().as_str() {
        "new york city" => println!("New York City is the largest city in the United States, with a population of over 8 million people. It is a global center for finance, media, art, and culture."),
        "london" => println!("London is the capital and largest city of England and the United Kingdom, with a population of over 8 million people. It is a global center for finance, education, healthcare, media, and the arts."),
        "paris" => println!("Paris is the capital and largest city of France, with a population of over 2 million people. It is known as the City of Light, and is famous for its art, fashion, cuisine, and historic landmarks such as the Eiffel Tower and the Louvre Museum."),
        "tokyo" => println!("Tokyo is the capital and largest city of Japan, with a population of over 13 million people. It is a global center for finance, technology, entertainment, and fashion."),
        _ => println!("Sorry, I don't have an introduction for that city."),
    }
}
