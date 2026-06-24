//Since both front_of_house and eat_at_restaurant are siblings, no need to make front_of_house
//public to access its contents. Though, its contents do have to be public to be accessed.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//Use goes out of scope when new Mod is created.(mod creates its own scope.)
//to make use of this link, you can add it inside the customer module
//or use 'super' within parent module.
use crate::front_of_house::hosting;

//we can also use re-exporting to access add_to_waitlist from other places into their scope
//pub use crate::front_of_house::hosting;

//mod customer {
//can add 'use crate:;front_of_house:;hosting' here.

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //Order a Breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //used super here
    //super::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}
//}

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
