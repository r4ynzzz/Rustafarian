use std::io;

fn main() {
    //Data Types: Scalar and Compound
    /*Scalar Types:*/

    //Integer Types
    let _mine:u32 = 23;
    let right = 32;

    //Floating-Point types:
    let _x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //Numeric Op's
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //Boolean:
    let t = true;
    let f: bool = false; // with explicit type annotation

    //Character types:
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /*Compound Types:*/
    //Tuple Type:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //get values from tuple:
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    //Another way:
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Array Types
    //Arrays in Rust are fixed. Use Vectors if you need collection that grows.
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    //Write array type:
    //Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //initialize an array to contain the same value for each element
    let a = [3; 5];

    //Access Array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    //Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
