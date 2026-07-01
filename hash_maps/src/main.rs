fn main() {
    //Creating a new HashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    //HashMaps have keys of type String and values of type i32.
    scores.insert(String::from("Barcelona"), 1);
    scores.insert(String::from("Real Madrid"), 5);

    //Accessing Values in a Hash Map.
    //The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.
    //This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>,
    //then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
    let team_name = String::from("Real Madrid");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //We can iterate over each k-v pair using a for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //Managing Ownership in hash maps.
    //For types that implement the Copy trait, like i32, the values are copied into the hash map.
    //For owned values like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Countryside");
    let field_value = String::from("France");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //field_name and field_value are invalid from here.
    //
    //If we insert references to values into the hash map, the values won’t be moved into the hash map.
    //The values that the references point to must be valid for at least as long as the hash map is valid.

    //Updating a Hash Map
    //
    //Overwriting a value
    //if we insert a key and a value into a hash map and then insert that same key with a different value,
    //the value associated with that key will be replaced.
    scores.insert(String::from("Barcelona"), 2);
    scores.insert(String::from("Barcelona"), 3);

    println!("{scores:?}");

    //Adding a key and value only if a key isnt present
    //Use "entry" that takes the key you want to check as a parameter.
    //The return value of the entry method is an enum called Entry that represents a value that might or might not exist
    scores.entry(String::from("Atletico Madrid")).or_insert(1);
    scores.entry(String::from("Real Madrid")).or_insert(2);

    println!("{scores:?}");

    //Updating a value based on the old value
    //
    //code that counts how many times each word appears in some text.
    let text = "Hola mundo maravilloso mundo";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
