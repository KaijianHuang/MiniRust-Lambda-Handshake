use rand::seq::SliceRandom;

fn main() {
    let cookie_messages = vec![
        "You will have a great day today!",
        "Good news is coming your way!",
        "You will soon receive a promotion!",
        "The stars will align in your favor!",
        "You will find true love soon!",
        "Your creativity will blossom!",
        "You will receive unexpected blessings!",
        "Your hard work will pay off!",
    ];

    let mut rng = rand::thread_rng();
    let message = cookie_messages.choose(&mut rng).unwrap();
    println!("Lucky cookie message: {}", message);
}
