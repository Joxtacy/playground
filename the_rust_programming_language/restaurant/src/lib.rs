#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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
}

mod front_of_house; // this mod is now a separate crate in front_of_house.rs

// Absolute path
pub use crate::front_of_house::hosting; // The `pub` allows external code to call `hosting` as well
// Relative path
// use self::front_of_house::hosting;

// Specify the parent when using use for bringing functions into scope
// When bringing structs, enums, and other items into scop with use, specify the full path
use std::collections::HashMap;

// We can nest bringing items into scope
use std::{cmp::Ordering, io};
// which is the same as
// use std::io;
// use std::cmp::Ordering;
// Another example bringing in io and Write
// use std::io::{self, Write};

// The exception would be if bringing two items in scope which have the same name
use std::fmt;
// use std::io;
// Could be solved by specifying an alias
use std::fmt::Result;
use std::io::Result as IoResult;
// fn function1() -> fmt::Result {
    // --snip--
// }

// fn function2() -> io::Result<()> {
    // --snip--
// }

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // With `use` to bring hosting into scope
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}
