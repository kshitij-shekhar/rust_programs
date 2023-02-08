//Program to generate the nth fibonacci number
use std::io;
fn main() {
    //P.S Error handling as a beginner in rust seems to be unnecessarily verbose, probably because I'm still a noob
    let n:u32=loop{
    println!("Please enter n");
    let mut n=String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n:u32=match n.trim().parse() {
        Ok(num) => break num,
        Err(_) => continue,
    };
    
    };
    let result:u32=fib(n);
    println!("the {n}th fibonacci number is {result}");
}

fn fib(n:u32) -> u32{
    //recursively compute
    if n<=1{
        return n;
    }
    return fib(n-1)+fib(n-2);
}
