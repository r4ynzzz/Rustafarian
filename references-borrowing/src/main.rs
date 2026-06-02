fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn mut_reference() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope
    let r2 = &mut s;
}

fn bad_mut_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}

fn better_mut_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

//Rules
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
