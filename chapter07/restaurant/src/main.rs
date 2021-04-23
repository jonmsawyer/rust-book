//use std::fmt;
//use std::io;
//use std::fmt::Result;
//use std::io::Result as IoResult;
use std::collections::HashMap;

use rand::Rng;

//use restaurant; // import unneeded, as it's part of the lib crate


//fn function1() -> fmt::Result {
//    // --snip--
//}
//
//fn function2() -> io::Result<()> {
//    // --snip--
//}

fn main() {
    restaurant::eat_at_restaurant();
    
    let mut map = HashMap::new();
    map.insert(1, 2);
    
    println!("map = {:?}", map);
    
    //let res = function1();
    //let res2 = function2();
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number = {}", secret_number); 
}
