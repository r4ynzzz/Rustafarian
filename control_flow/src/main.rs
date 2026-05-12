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

    //control();
    //disambiguate();
    //while_condition();
    for_condition();
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

fn disambiguate(){
    /* Disambiguating with Loop Labels */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}")
}

/* Streamlining Conditional Loops with while */
fn while_condition(){
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("BANKAI!!!!!!")
}

//Better way of above while condition.
fn nicer_condition(){
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("BANKAI!!!!!!")
}

/* Looping Through a Collection with for */
fn for_condition(){
    //Using while loop
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //using for loop
    for element in a {
        println!("the value is: {element}");
    }
}