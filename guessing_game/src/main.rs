use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Print to console
    println!("Guessing Game");

    println!("Secret number: {secret_number}");

    loop {
        println!("Guess a number: ");
        // Declare a mutable string
        let mut guess = String::new();
        // Read from console
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed, {guess}");
    }
}
