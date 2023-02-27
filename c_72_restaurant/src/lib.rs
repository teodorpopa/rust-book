pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> String {
            return String::from("Customer added to waitlist!");
        }
    }
}

pub mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}