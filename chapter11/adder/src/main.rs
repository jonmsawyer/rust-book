use adder::Guess;

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
    foo::beef::qux();
    foo::deadbeef();
    
    let guess = Guess::new(99);
    println!("{}", guess.value());
}
