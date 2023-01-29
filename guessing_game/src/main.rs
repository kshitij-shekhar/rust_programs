use rand::Rng; //Rng trait brings into scope functions needed to generate Random numbers
use std::io;
use std::cmp::Ordering; //Ordering type is an enum with variants Less,Greater and Equal
fn main() {
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1..=10);
    // println!("The secret number is : {secret_number}");
    loop{
        println!("Please enter your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_int:u32= match guess.trim().parse() {//.expect("Please enter a number!"); 
            Ok(num) => num,
            Err(_) => continue,
    };
        //.parse() returns a Result type, which is an enum

        match guess_int.cmp(&secret_number){ //the call to cmp will return Ordering::Less or Ordering::Greater or Ordering::Equal,
            //match uses the return value to check if it matches any patterns in its arms
            Ordering::Less => println!("Too small"), //arm1
            Ordering::Greater => println!("Too big"), //arm2
            Ordering::Equal => {println!("You win!"); break;} //arm3
        };

    }
    
    //stdin returns an instance of io::Stdin which is a type that represents a handle to standard input
    //read_line appends the std input to a string specified as argument to the method
    // println!("Address of guess : {&guess}");
    //.expect() returns a Result value which is an enum. could be Ok or Err variant
    //Ok and Err and instances of Result, and have corresponding methods defined

    //Ok contains the number of bytes of user input
    // println!("You guessed : {guess}");
}
