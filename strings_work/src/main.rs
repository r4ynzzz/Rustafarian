use std::array::from_fn;

fn main() {
    //String, in Rust, is string slice 'str' at its core.
    //'String' is provided by Rust's standard library, not built into the core language.
    //When refering to "Strings", it could be 'String' or 'str'.

    //Creating a String
    //"String" is implemented as a wrapper around a vector of bytes, so
    //many of the same operations found in Vec<T> are avilable with "String."
    let mut s = String::new();

    //"to_string" is avilable on any type that implements the 'Display' trait.
    //We use the 'to_string' method to create a 'String' from a string literal('str').
    let data = "initial contents";
    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    //We can also use the function 'String::from' to create a 'String' from a string literal.
    let s = String::from("initial contents");
    let hello = String::from("Hola");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");

    //Updating String
    //Using 'push_str'. It does not take ownership of the parameter.
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //Using 'push'. method takes a single character as a parameter and adds it to the 'String.'
    let mut s = String::from("lmfa");
    s.push('o');
}
