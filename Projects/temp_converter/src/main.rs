use std::io;

fn main() {
    println!("Welcome to the Temperature converter.");
    loop{
        conversion();
        println!("Do you want to conver another value? Yes(Y) or No(N): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read type");

        if choice.trim().to_lowercase() == "y"{
            conversion();
        } else {
            println!("Thank you for using this application.");
            break;
        }
    }
}

fn celsius(x: f32){
    let x = x - 32.0;
    let x = x / 1.8;
    println!("Your conversion to Celsius is: {x}")
}

fn fahrenheit(x: f32){
    let x = x * 1.8;
    let x = x + 32.0;
    println!("Your conversion to Fahrenheit is: {x}")
}

fn conversion(){
    loop{
        println!("Which Type do you wish to convert to? Celsius(C) or Fahrenheit(F)");
        let mut temp_choice = String::new();
        io::stdin()
            .read_line(&mut temp_choice)
            .expect("Failed to read type");

        println!("Enter the temperature value to convert: ");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature");
        
        let temp: f32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_choice.trim().to_lowercase() == "c" {
            celsius(temp);
        } else if temp_choice.trim().to_lowercase() == "f" {
            fahrenheit(temp);
        } else {
            println!("Invalid option!");
        }
        break;
    }
}