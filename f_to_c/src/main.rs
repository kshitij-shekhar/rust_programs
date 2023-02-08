//Program to convert Fahrenheit to Celsius 
use std::io;
fn main() {
    //take F input 
    // The match expression will decide what to return to f_temp from the loop
    let f_temp :f32=loop{
        println!("Please enter the Fahrenheit temperature! ");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        //use match for error handling
        //If the user has entered a number, break from the loop and return the number that 
        //was parsed
        //If the parsing doesn't work, restart the loop's iteration
        let temp:f32= match temp.trim().parse() {
            Ok(num) =>  break num,
            Err(_) => continue,
        
    };

    
    };
    let result:f32=convert_f_to_c(f_temp);
    println!("{f_temp} in Celsius : {result}");

}


fn convert_f_to_c(f : f32) -> f32{
    (f-32.0)*(5.0/9.0)
}