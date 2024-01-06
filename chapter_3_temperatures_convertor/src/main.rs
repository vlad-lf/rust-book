use std::io;

fn main() {
    println!("Hello and welcome to the temperature convertor!");

    loop {
        println!("Please, select the input type: 'C' for Celcius or 'F' for Fahrenheit");

        let mut temp_unit = String::new();

        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Failed to read line");

        let temp_unit = temp_unit.trim();
        if temp_unit == "C" || temp_unit == "F" {
            println!("You've chosen {temp_unit}, thanks!");
        } else {
            continue;
        }

        println!("Please, enter the temperature in {temp_unit} you'd like to be converted");

        let mut temp_value = String::new();
        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read line");

        let temp_value: f64 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_unit == "C" {
            println!("Converted: {temp_value}C is {:.2}F", celcius_to_fahrenheit(temp_value));
        } else if temp_unit == "F" {
            println!("Converted: {temp_value}F is {:.2}C", fahrenheit_to_celcius(temp_value));
        }
    }
}

fn fahrenheit_to_celcius(temperature: f64) -> f64 {
    ((temperature) - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0 / 5.0) + 32.0
}
