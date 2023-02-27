use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;

fn main() {

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{:?}", eat_at_restaurant());
}

pub fn eat_at_restaurant() -> String {

    let order1 = c_72_restaurant::back_of_house::Appetizer::Soup;
    let order2 = c_72_restaurant::back_of_house::Appetizer::Salad;

    println!("Order 1: {:?}", order1);
    println!("Order 2: {:?}", order2);

    return c_72_restaurant::front_of_house::hosting::add_to_waitlist();
}