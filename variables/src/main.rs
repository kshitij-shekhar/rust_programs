fn main() {
    // println!("Hello, world!");
    //Variables in rust are by default immutable
    // let mut x=5;
    // println!("The value of x is : {x}");
    // x=6;
    // println!("The value of x is: {x}");
    //Shadowing a var requires repeating the let keyword
    let x=5;
    let x=x+1;
    {
        let x=x*2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");
    //Shadowing allows changing the variable's type, but mut only allows changing its value
    let boolean :bool=true;
    println!("{boolean}");
    // let z: char='Z';
    // let tup:(i32,f64,u8,char,str)=(500,6.57,2,'Z',"hello");
    let tup:(i32,f64,u8,char,&str)=(500,6.57,2,'Z',"hello");
    let (a,b,c,d)=tup;
    println!("{:?}",tup);
    let arr:[i32;5]=[1,2,3,4,5]; //array definition
    let a=[3;5]; // same as let a=[3,3,3,3,3]
    
}
