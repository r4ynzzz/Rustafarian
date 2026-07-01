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

    //Concatenating with '+' and 'format.'
    //Using '+' operator.
    //We can only add a string slice to a String; we can’t add two String values together.
    //Basically, s1 goes out of scope after s3 and we reference s2 because of ownership
    //the add function works. Even though s2 is a String, it get coerced into a str when
    //add function is called.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    //This method gets a bit complicated, best to use the 'format!' macro.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //Using 'format!'
    //The format! macro works like println!, but instead of printing the output to the screen,
    //it returns a String with the contents
    let s1 = String::from("rock");
    let s2 = String::from("paper");
    let s3 = String::from("scissors");

    let s = format!("{s1}-{s2}-{s3}---SHOOT!!!");

    //Indexing into Strings
    //A String is a wrapper over a Vec<u8>
    //len will be 4, which means the vector storing the string "Hola" is 4 bytes long
    let hello = String::from("Hola");

    //If you were asked how long the string is, you might say 12. In fact, Rust’s answer is 24:
    //That’s the number of bytes it takes to encode “Здравствуйте” in UTF-8,
    //because each Unicode scalar value in that string takes 2 bytes of storage
    //An index into the string’s bytes will not always correlate to a valid Unicode scalar
    let hello = String::from("Здравствуйте");

    //3 ways to look at Rust's strings: as bytes, scalar values and grapheme clusters
    //
    // “नमस्ते”" is stored as vector of u8:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //
    // as Unicode scalar type:
    // ['न', 'म', 'स', '्', 'त', 'े'] - refer to the book.
    //
    // as grapheme clusters:
    // ["न", "म", "स्", "ते"]
    //
    // A final reason Rust doesn’t allow us to index into a String to get a character is that
    // indexing operations are expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a String,
    // because Rust would have to walk through the contents from the beginning
    // to the index to determine how many valid characters there were.

    //Slicing Strings
    //Rather than indexing using [] with a single number, we use [] with a range to
    //create a string slice.
    let answer = &hello[0..4]; //contains the first 4 bytes fo the string. since each
    //character here is 2 bytes, we get "Зд"
    //if we try to slice only a part of a characters bytes,
    //Rust will panic.

    //Iterating Over Strings:
    //The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
    //For individual Unicode scalar values, use the chars method.
    for c in "Зд".chars() {
        println!("{c}");
    }

    //bytes method returns raw bytes
    for c in "Зд".bytes() {
        println!("{c}");
    }

    //Go through documentation for more methods like "contains" for searching
    //and "replace" for subbing parts for another.
}
