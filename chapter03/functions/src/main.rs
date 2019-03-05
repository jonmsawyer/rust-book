#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    
    another_function(5, 6);
    
    let x = { let y = 3; y };
    println!("x: {}", x);
    
    let five = five();
    
    println!("five: {}", five);
    
    let plus_one = plus_one(10);
    /* is this comment
       legal? */
    println!("10 plus_one: {}", plus_one);
}

fn another_function(x: i32, y: i32) {
    println!("Another function!");
    println!("x: {}, y: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
