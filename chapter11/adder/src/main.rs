//use adder::*;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        Guess {
            value
        }
    }
}

mod foo {
    mod bar {
        pub fn beef() {
            println!("foo::bar::beef");
        }
    }
    
    pub mod beef {
        pub fn qux() {
            super::bar::beef();
        }
    }
    
    pub fn deadbeef() {
        println!("Is this accessible?");
    }
}

fn main() {
    use foo;
    foo::beef::qux();
    foo::deadbeef();
    
    use Guess;
    let guess = Guess::new(99);
    println!("{}", guess.value);
}
