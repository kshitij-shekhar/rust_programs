fn main() {
    // another_function(5);
    // let y={
    //     let x=3;
    //     x+1 //; at the end of an expression = statement, expressions return/evaluate to some value
    //     // statements do not return values
    //     // functions are also expressions
    // };
    // println!("Y = {:?}",y);
    // let x=five();
    // println!("The value of x is {:?}",x);

    // let condition=false;
    // let x = if condition {5} else {6};
    // println!("x : {x}");
    let mut counter=0;
    let result=loop{
        counter +=1;
        if counter==10{
            break counter*2;
        }
    
    };
    println!("the value of result is {result}");

    for number in (1..4).rev(){ //1..4 is a Range type, .rev() reverses it, last number not included
        println!("{number}!");
    }
    println!("LIFTOFF");
}

// fn another_function(x:i32){
//     println!("The value of x is {x}");
// }

fn five() -> i32{//return type
    // 5 //expression, implicitly rust functions return the last expression, can also use
    // return keyword
    return 5;
}
