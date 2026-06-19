#[derive(Debug)]
enum UsaState {
    Georgia,
    California,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    //binding value
    Quarter(UsaState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //if you want to run multiple lines of code, use curly brackets
        /*
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        */
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn game() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //catch-all pattern
        other => move_player(other),
        //pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern
        //_ is a special pattern that matches any value and does not bind to that value.
        _ => (),
    }
}

fn add_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn remove_fancy_hat() {}

fn main() {
    value_in_cents(Coin::Quarter(UsaState::Georgia));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
