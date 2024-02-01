use std::{char, io};

fn main() {
    println!("Convert temperature!");

    println!("Please choose your base temperature. \n a) Celsius  \n b) Fahrenheit");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: char = input.trim().parse::<char>().expect("Please type a char!");

    let mut current_temp = "";
    match input {
        'a' => {
            println!("You chose: Celsius");
            current_temp = "Celsius";
        }
        'b' => {
            println!("You chose: Fahrenheit");
            current_temp = "Fahrenheit";
        }
        _ => println!("You chose: {input}"),
    }
    println!("How many degrees?");

    let mut degrees = String::new();
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");
    let degrees: f32 = degrees.trim().parse().expect("Please type a number!");

    match current_temp {
        "Celsius" => {
            let fahrenheit = (degrees * 9.0 / 5.0) + 32.0;
            println!("It's {fahrenheit} Fahrenheit!");
        }
        "Fahrenheit" => {
            let celsius = (degrees - 32.0) * 5.0 / 9.0;
            println!("It's {celsius} Celsius!");
        }
        _ => println!("Whoops"),
    }
}
