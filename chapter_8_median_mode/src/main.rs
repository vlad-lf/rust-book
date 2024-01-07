use std::io;

fn main() {
    println!("Welcome to the Median & Mode solver!\nPlease, enter the array of unsigned integers spliited by ','");

    let mut arr = String::new();

    io::stdin()
        .read_line(&mut arr)
        .expect("Failed to read line");

    println!("You've entered {arr}");
}