mod front_of_house;
mod back_of_house;

//use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;
use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // Using the 'use' import for `hosting`
    hosting::add_to_waitlist();
    
    // Using the full 'use' import for `add_to_waitlist`
    add_to_waitlist();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("{:?}", meal);

    // The next two lines won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes with the
    // meal.
    //meal.seasonal_fruit = String::from("blueberries");
    //println!("{}", meal.seasonal_fruit);
    
    let soup = back_of_house::Appetizer::Soup(String::from("Potato"));
    println!("{:?}", soup);
    
    let salad = back_of_house::Appetizer::Salad(String::from("Ceasar"));
    println!("{:?}", salad);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
