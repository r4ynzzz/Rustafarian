fn main() {
    //Rust variables immutable by default.
    //Add "mut" to make mutable.
    let x = 5;
    println!("The value of x is: {x}");

    //This is called Shadowing.
    //applying a different value to the same variable by uing let.
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    //Constant values cannot change
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}
