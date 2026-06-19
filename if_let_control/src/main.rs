fn main() {
    let config_max = Some(3u8);
    /* Results in 3, But the 'None' value boiler plate is added.
    * Can do this without boiler plate
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    */

    //Better Way - If let works the same as match.
    //But you lose the exhaustive checking match that makes
    //sure you dont forget handling cases.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
