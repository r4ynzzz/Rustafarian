//Since both front_of_house and eat_at_restaurant are siblings, no need to make front_of_house
//public to access its contents. Though, its contents do have to be public to be accessed.
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_watchlist();
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    //if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    //because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function
    //that constructs an instance of Breakfast (we’ve named it summer here).
    //If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant,
    //because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        //Use super incase module gets moved. Super, in this case, goes to parent module of
        //back_of_house.
        super::deliver_order();
    }

    fn cook_order() {}
}

use std::fmt::Result;
//use 'as' to give alias.
use std::io::Result as IoResult;

//fn function1() -> Result {}

//then, use alias
//fn function2() -> IoResult<()> {}

//Nested Paths.
//would be:
//
//use std::cmp::Ordering;
//use std::io;
//
//but now, with nesting:
use std::{cmp::Ordering, io};

//another Way of nesting paths.
//would be:
//
//use std::io;
//use std::io::Write;
//
//but now:
use std::io::{self, Write};

//if we want to bring all public items of a path, use '*'
use std::collections::*;
