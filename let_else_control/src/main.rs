use std::fmt::format;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Georgia,
    California,
    Texas,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Georgia => year >= 1819,
            UsState::Texas => year >= 1959,
            UsState::California => year >= 1969,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let b = describe_state_quarter(Coin::Quarter(UsState::Texas));

    println!("{b:?}");
}
