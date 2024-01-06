use std::io;

fn main() {
    println!("Hello and welcome to the n'th Fibonaci number calculator!");

    loop {
        println!("Please, input n:");

        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u16 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Fibonaci's {n} number is {}", get_fibonacci(n));
    }
}

fn get_fibonacci(n: u16) -> u64 {
    if n < 2 {
        n.into()
    } else {
        let mut fibonacci_last_3 = (0, 1, 1);
        let mut mut_current_index = 2;
        while mut_current_index < n {
            mut_current_index += 1;
            fibonacci_last_3 = (
                fibonacci_last_3.1,
                fibonacci_last_3.2,
                fibonacci_last_3.1 + fibonacci_last_3.2
            )
            
        }
        fibonacci_last_3.2
    }
}