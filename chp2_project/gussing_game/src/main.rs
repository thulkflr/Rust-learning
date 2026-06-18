use std::io;
use rand::RngExt;
use std::cmp::Ordering;


fn main() {

let mut selected_option = String::new(); 

let secret_number = rand::rng().random_range(1..=100);//generate a random number between 1 and 100, inclusive.
let mut guess = String::new(); //mutable variable.
// AND it is a empty string, we will store the user input in this variable.

// let guess = String::new(); Immutable Variable.

//mut means mutable, we can change the value of guess 
//Because the variables in Rust are immutable by default, we need to use mut to make it mutable.
//Immutability is a feature of Rust that helps prevent bugs and makes the code more predictable.
//so we cant change the value of guess without mut, if we try to change it without mut, we will get an error.

println!("Please Enter Your Option: ");
println!("1. Guess the number");
println!("2. Compare the numbers");
println!("3. Multiple guesses with looping");

io::stdin()
    .read_line(&mut selected_option)
    .expect("Failed to read line");

match selected_option.trim() {
    "1" => guessNumber(secret_number, &mut guess),
    "2" => compareNumbers(secret_number, &mut guess),
    "3" => multiguessesWithLooping(secret_number, &mut guess),
    _ => println!("Invalid option"),
}


// guessNumber(secret_number, &mut guess); //pass the secret number and the mutable reference of guess to the function guessNumber


}


fn guessNumber(secret_number: u32,  guess:&mut String)->(){
println!("Guess the number!");

println!("the sectet number is: {secret_number}");
println!("Please input your guess.");
io::stdin()
    .read_line(guess)
    .expect("wrong read line");
//read_line is a method that reads a line of input from the user and stores it in the guess variable.

//&mut guess is a reference to the guess variable, we need to use &mut because read_line needs a mutable reference to the variable where it will store the input.
//we use reference because we want to pass the variable to the function, and we want to allow the function to modify the variable, so we use &mut to pass a mutable reference to the variable.
//why we not use mut guess instead of &mut guess? because read_line needs a reference to the variable, not the variable itself. if we use mut guess, we will get an error because read_line expects a reference, not a value.
//why we not use &guess instead of &mut guess? because read_line needs a mutable reference to the variable, if we use &guess, we will get an error because read_line expects a mutable reference, not an immutable reference.

//expect is a method that is used to handle errors. If read_line fails, it will return an error, and expect will panic and print the message "wrong read line".

    println!("You guessed: {guess}");
}

fn compareNumbers(secret_number: u32, guess: &mut String) -> () {
    println!("comparing numbers");
    io::stdin()
        .read_line(guess)
        .expect("wrong read line");
    let guess: u32 =guess.trim().parse().expect("Please type a number!");//trim is a method that removes whitespace from the beginning and end of a string. parse is a method that converts a string to a number. expect is a method that is used to handle errors. If parse fails, it will return an error, and expect will panic and print the message "Please type a number!".
    //wait a minute, why we create a new variable guess with the same name as the parameter guess?
    // because we want to shadow the parameter guess with a new variable guess that has the type u32.
    // Shadowing is a feature of Rust that allows us to declare a new variable with the same name as a previous variable, and the new variable will shadow the previous variable. This is useful because we want to convert the string guess to a number, and we want to use the same name for the new variable to make it easier to read and understand the code.
    // shaddowing is reusing the same name for a variable, but with a different type or value. In this case, we are reusing the name guess for a new variable that has the type u32, which is the type of the secret number. This allows us to compare the guess with the secret number without having to use a different name for the variable.
    
    //    exapmple of shadowing:
    //     let spaces = "   ";
    //     let spaces = spaces.len();
    //     the shadow is immutable, we can not change the value of spaces after we shadow it, but we can shadow it again if we want to change its value.
    
    //we need to use parse because read_line returns a string, and we need to convert it to a number to compare it with the secret number.


    println!("You guessed: {guess}");

    //match is a control flow operator that allows us to compare the guess with the secret number and print a message based on the result of the comparison.
    match guess.cmp(&secret_number) {//cmp is a method that compares two values and returns an Ordering enum. The Ordering enum has three variants: Less, Greater, and Equal.
        //cmp receives a string reference, so we need to use &secret_number to pass a reference to the secret number.
        Ordering::Less => println!("Too small!"),//Less means the guess is less than the secret number.
        Ordering::Greater => println!("Too big!"),//Greater means the guess is greater than the secret number.
        Ordering::Equal => println!("You win!"),//Equal means the guess is equal to the secret number.
    }

}

fn multiguessesWithLooping(secret_number: u32, guess: &mut String)->(){
    println!("Guess the number!");
   

    loop{
        println!("please input your guess");
            guess.clear();//clear is a method that clears the contents of the string, we need to clear the guess variable before reading a new line of input from the user, because read_line appends the input to the existing string, so if we don't clear it, we will get a string that contains all the previous guesses.
            io::stdin()
        .read_line(guess)
        .expect("wrong read line");
    
    // let guess: u32 =guess.trim().parse().expect("Please type a number!");
    
    let guess: u32 = match guess.trim().parse() { // use match to handle the error instead of expect, so that we can continue the loop if the user inputs an invalid number.
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            continue;
        }
    };//trim is a method that removes whitespace from the beginning and end of a string. parse is a method that converts a string to a number. expect is a method that is used to handle errors. If parse fails, it will return an error, and expect will panic and print the message "Please type a number!".
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {//cmp is a method that compares two values and returns an Ordering enum. The Ordering enum has three variants: Less, Greater, and Equal.
        //cmp receives a string reference, so we need to use &secret_number to pass a reference to the secret number.
        Ordering::Less => println!("Too small!"),//Less means the guess is less than the secret number.
        Ordering::Greater => println!("Too big!"),//Greater means the guess is greater than the secret number.
        Ordering::Equal => {
            println!("You win!");//Equal means the guess is equal to the secret number.
                break;//break is a control flow operator that allows us to exit the loop when the user wins.
        },
    }
}
}