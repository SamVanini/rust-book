use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Random generator local to the current execution thread
    // seeded by the OS
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // let apples = 5;
        // Rust variables are immutable by default

        // String::new, :: indicates that new is an
        // associated function (it is implementing the type)
        // of String type
        let mut guess = String::new();

        // Alternatively, std::io::Stdin without the initial import
        io::stdin() // standard input function
            .read_line(&mut guess) // & states that the argument is a reference
            .expect("Failed to read line"); // error handling, if missing compiler warning

        // read_line puts whatever the user enters into the string
        // we pass to it but it also returns a Result value.
        // It is an enum encoding error-handling info (Ok/Err)
        // Result variants have methods encode into them

        // Shadowing
        // Trim removes carriage return and new line chars
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
