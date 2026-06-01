fn main() {
    let mut s = String::from("Hello");
    takes_ownership(s);
    //println!("{s}"); //will not work. value borrowed, therefore out of scope.
    s = String::from("ahoy");
    new_ownership(s);

    let s1 = gives_ownership(); // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn new_ownership(new_string: String) {
    println!("{new_string}");
}

fn scopage() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); //s1 now out of dropped and out of scope. Would have to use s2.
    //
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
