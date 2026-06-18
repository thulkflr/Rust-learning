use std::io;
use rand::RngExt;

fn main() {

println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);//generate a random number between 1 and 100, inclusive.
println!("the sectet number is: {secret_number}");
println!("Please input your guess.");

let mut guess = String::new(); //mutable variable.
// AND it is a empty string, we will store the user input in this variable.

// let guess = String::new(); Immutable Variable.

//mut means mutable, we can change the value of guess 
//Because the variables in Rust are immutable by default, we need to use mut to make it mutable.
//Immutability is a feature of Rust that helps prevent bugs and makes the code more predictable.
//so we cant change the value of guess without mut, if we try to change it without mut, we will get an error.
io::stdin()
    .read_line(&mut guess)
    .expect("wrong read line");
//read_line is a method that reads a line of input from the user and stores it in the guess variable.

//&mut guess is a reference to the guess variable, we need to use &mut because read_line needs a mutable reference to the variable where it will store the input.
//we use reference because we want to pass the variable to the function, and we want to allow the function to modify the variable, so we use &mut to pass a mutable reference to the variable.
//why we not use mut guess instead of &mut guess? because read_line needs a reference to the variable, not the variable itself. if we use mut guess, we will get an error because read_line expects a reference, not a value.
//why we not use &guess instead of &mut guess? because read_line needs a mutable reference to the variable, if we use &guess, we will get an error because read_line expects a mutable reference, not an immutable reference.

//expect is a method that is used to handle errors. If read_line fails, it will return an error, and expect will panic and print the message "wrong read line".

    println!("You guessed: {guess}");

}