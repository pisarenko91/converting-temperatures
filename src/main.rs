use std::io;

// fn convert_temp(temp: f64) -> f64 {}

fn main() {
    println!("Welcome to temp converter in Rust!");
    println!(
        "Select to which dimension you are converting, type 'C' for Celsius or 'F' for Fahrenheit"
    );

    let mut type_of_conversion = String::new();

    io::stdin()
        .read_line(&mut type_of_conversion)
        .expect("Failed to read the input.");

    let type_of_conversion = type_of_conversion.trim().to_uppercase();

    let mut input_temp = String::new();
    if type_of_conversion == "C" {
        println!("You have selected to convert from Fahrenheit to Celsius!");
        println!("Type in desired temperature in Fahrenheit:");
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read the input.");

        let input_temp = input_temp.trim().parse::<f64>().unwrap();

        let converted_temp: f64 = (input_temp - 32.0) * 5.0 / 9.0;

        println!(
            "{} degrees in Fahrenheit is {} degrees in Celsius.",
            input_temp, converted_temp
        );
    } else if type_of_conversion == "F" {
        println!("You have selected to convert from Celsius to Fahrenheit!");
        println!("Type in desired temperature in Celsius:");
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read the input.");

        let input_temp = input_temp.trim().parse::<f64>().unwrap();

        let converted_temp: f64 = (input_temp * 9.0 / 5.0) + 32.0;

        println!(
            "{} degrees in Celsius is {} degrees in Fahrenheit.",
            input_temp, converted_temp
        );
    } else {
        println!("You have not selected any of the valid options 'F' or 'C', please re-run the program and choose valid option!")
    }
}
