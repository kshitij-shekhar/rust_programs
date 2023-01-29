use rand::Rng;//Rng trait brings into scope functions needed to generate Random numbers
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1..=10);
    println!("The secret number is : {secret_number}");
    println!("Please enter your guess!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //stdin returns an instance of io::Stdin which is a type that represents a handle to standard input
    //read_line appends the std input to a string specified as argument to the method
    // println!("Address of guess : {&guess}");
    //.expect() returns a Result value which is an enum. could be Ok or Err variant
    //Ok and Err and instances of Result, and have corresponding methods defined

    //Ok contains the number of bytes of user input
    println!("You guessed : {guess}");
}
