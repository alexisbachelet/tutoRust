use rand::Rng; // Random, range.
use std::cmp::Ordering; // Compare.
use std::io; // Standard Input, Output

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Re assign a new variable to change it's type.
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /*
        Remember that parse returns a Result type
        Result is an enum that has the variants Ok and Err
        Ok value that contains the resultant numbe

        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catchall value;
            // match all Err values
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // Comare guess with the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
