fn main() {
    //create Vector
    //let mut v = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    //Update Vector with push
    //v.push(5);
    //v.push(6);
    //v.push(7);
    //v.push(8);

    //Two ways of reading values stored in vector: via indexing or by using 'get' method
    //indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    //Get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //Causes a panic, indexoutofbounds error
    let does_not_exist = &v[100];
    //returns None, does not panic.
    let does_not_exist = v.get(100);

    //next section wont compile. Since vectors are stored on the heap,
    //making a reference and adding a value to the vector cannot happen
    //at the same time, based on the rules of borrowing and referencing.
    //The reference would now point to deallocated memory, since adding values
    //to vector might require allocating new memory and copying ol element into
    //space.
    let first = &v[0];

    //v.push(6);

    //use 'first' before pushing new element
    println!("The first element is: {first}");

    //Iterating through values in Vector.
    for i in &v {
        println!("{i}");
    }

    //iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    //'*' is a dereference operator.
    for i in &mut v {
        *i += 50;
    }

    //Using an Enum to Store Multiple Types and accesing them in a Vector.
    //Vectors typiclly carry the same types. Since the type here would be
    //'SpreadSheetCell', we can bypass the 'same type' rule by placing
    //different types in enum 'SpreadSheetCell.'
    //We would then use a match expression to work with the data.
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    //vectors are also dropped when they go out of scope. The contens also get dropped.
}
