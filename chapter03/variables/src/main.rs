#![allow(unused_variables)]
fn main() {
    let x = 5;
    
    let x = x + 1;
    
    let x = x * 2;
    
    println!("The value of x is: {}", x);
    
    let spaces = "    ";
    let spaces = spaces.len();
    
    println!("The number of spaces is: {}", spaces);
    
    let guess: u32 = "42".parse().expect("Not a number!");
    
    println!("Guess is: {}", guess);
    
    let x = 2.0; // f64
    
    let y: f32 = 3.0; // f32
    
    println!("X and Y are now: {}, {}", x, y);
    
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
    
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    //println!("tup: {}", tup);
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0;
    
    let six_point_four = x.1;
    
    let one = x.2;
    
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);
    
    let a = [1, 2, 3, 4, 5];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    //println!("a: {}", a);
    println!("3rd month: {}", months[2]);
}
