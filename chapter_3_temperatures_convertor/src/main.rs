use std::io;

fn main() {
    println!("Hello and welcome to the temperature convertor!");

    loop {
        println!("Please, select the input type: C for Celcius or F for Fahrenheit");

        let mut temp_unit = String::new();

        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Failed to read line");

        let temp_unit = temp_unit.trim();
        if temp_unit == "C" || temp_unit == "F" {
            println!("You've chosen {temp_unit}, thanks!");
            break;
        }
    }
}
