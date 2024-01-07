use std::io;

#[derive(Debug)]
enum Command {
    Help,
    Exit,
    NotImplemented
}

impl Command {
    fn parse_command(command_line: &str) -> Command {
        let command_line = command_line.trim().to_lowercase();
        if command_line == "help" {
            Command::Help
        } else if command_line == "exit" {
            Command::Exit
        } else {
            Command::NotImplemented
        }
    }
}

fn main() {
    println!("Hello and welcome to the Human Department management command-line interface!");
    println!("Please, input the command");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = Command::parse_command(&input[..]);

        match command {
            Command::Exit => break,
            Command::Help => println!("Use `exit` to exit the program"),
            Command::NotImplemented => println!("Unknown command, try again or try using 'help' command to see available options"),
        }

    }
}
