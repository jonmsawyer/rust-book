#[allow(dead_code)]
#[derive(Debug)]
pub enum Appetizer {
    Soup(String),
    Salad(String),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

#[allow(dead_code)]
fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();
}

#[allow(dead_code)]
fn cook_order() {}
