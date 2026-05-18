fn main() { 
    let mut s = String::from("Hello");

    takes_ownership(s);

    s = String::from("ahoy");

    new_ownership(s);
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn new_ownership(new_string: String){
    println!("{new_string}");
}