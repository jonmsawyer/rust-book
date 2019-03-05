#![allow(unused_variables)]

fn main() {
    let x = 7;
    
    if x > 7 {
        println!("The condition was true.");
    } else {
        println!("The condition was false.");
    }
    
    // wont work
    /*
    if x {
       ^ - must evaluate to bool
        println!("The condition is true.");
    }
    */
    
    if x != 0 {
        println!("number was something other than zero");
    }
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
    
    /* wont compile, return expressions must be of same type
    
    let condition = true;

    let number = if condition {
        5
    } else {
        "six" // expected integral variable, saw &str
    };

    println!("The value of number is: {}", number);
    */
    
    let condition = true;

    let number = if condition {
        5
    } else {
        10
    };

    println!("The value of number is: {}", number);
}
