fn main() {
    println!("Hello, world!");
    //say_and_express();
    //some_function();
    //another_function(5);
    //print_labelled_measurements(10, 'm');
    let x = five();

    println!("The value of x is: {x}");
}

fn some_function(){
    println!("I am a function")
}

//When using parameters, you must define each parameter.
fn another_function(x: i32){
    println!("The value of x is: {x}")
}

fn print_labelled_measurements(value:i32, unit_label:char){
    println!("The measurement is: {value}{unit_label}")
}

/* Statements and Expressions */
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.

fn say_and_express(){
    //Statement:
    let y= 6;

    let y = {
        //Expression: Expressions do not include ending semicolons.
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

/* Functions and Return Values */
//Set return with arrow. Return type must be specified. Can add 'return',(retrun 5), but not neccessary.
fn five() -> i32 {
    5
}