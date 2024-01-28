use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Welcome to Guessing Game");
 
    let secret_number = rand::thread_rng().gen_range(1..=50);

    loop {

        println!("Enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        // Convert string to uint
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Colder"),
            Ordering::Greater => println!("Hotter"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
