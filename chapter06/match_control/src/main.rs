#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c1 = Coin::Penny;
    println!("{:?} has value {}", &c1, value_in_cents(&c1));
    
    let c2 = Coin::Nickel;
    println!("{:?} has value {}", &c2, value_in_cents(&c2));
    
    let c3 = Coin::Dime;
    println!("{:?} has value {}", &c3, value_in_cents(&c3));
    
    let c4 = Coin::Quarter(UsState::Alaska);
    println!("{:?} has value {}", &c4, value_in_cents(&c4));
    
    let c5 = Coin::Quarter(UsState::Alabama);
    println!("{:?} has value {}", &c5, value_in_cents(&c5));
    
    let five = Some(5);
    println!("five = {:?}", five);
    
    let six = plus_one(five);
    println!("six = {:?}", six);
    
    let none = plus_one(None);
    println!("none = {:?}", none);
    
    let some_u8_value = Some(3u8);
    
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
    let mut count = 0;
    if let Coin::Quarter(state) = c1 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
    println!("count = {}", count);
}