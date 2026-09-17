use std::io;

fn main() {
    println!("Guess the number!");

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

    println!("You guessed: {guess}");
}
