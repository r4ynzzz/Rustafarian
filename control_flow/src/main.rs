fn main() {
    let number = 3;

    if number < 3 {
        println!("Condition was true");
    } else {
        println!("Condition was false")
    }

    /* IF in a Let statement */
    //Condition evals must also be of the same type.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");

    control();
}

fn control(){
    /* Repetition with Loops */
    //Loop include: loop, while, for.
    //Loop
    //loop{
        //println!("1A!")
    //}

    //Returning values from loops:
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}