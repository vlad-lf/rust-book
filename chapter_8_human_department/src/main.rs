use std::{io, collections::HashMap};

#[derive(Debug)]
enum Command {
    Help,
    Exit,
    Add { name: String, department: String },
    List,
    Unknown
}

impl Command {
    fn parse_command(command_line: &str) -> Command {
        let command_line = command_line.trim().to_lowercase();
        if command_line == "help" {
            Command::Help
        } else if command_line == "exit" {
            Command::Exit
        } else if command_line == "list" {
            Command::List
        } else if command_line.starts_with("add") {
            let add_command_split: Vec<&str> = command_line.split_whitespace().collect();
            Command::Add { name: add_command_split[1].to_string(), department: add_command_split[3].to_string() }
        } else {
            Command::Unknown
        }
    }
}

fn main() {
    println!("Hello and welcome to the Human Department management command-line interface!");
    println!("Please, input the command");

    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = Command::parse_command(&input);

        match command {
            Command::Help => println!("Use `exit` to exit the program\nUse `add X to Y` to add person `X` to department `Y`"),
            Command::List => println!("The current DB state is: {:#?}", database),
            Command::Add { name, department } => {
                println!("Adding {name} to {department}");
                match database.get_mut(&department) {
                    Some(dep_people) => {
                        dep_people.push(name);
                        dep_people.sort();
                    },
                    None => {
                        database.insert(department, vec![name]);
                    }
                };
            },
            Command::Exit => break,
            Command::Unknown => println!("Unknown command, try again or try using 'help' command to see available options"),
        }

    }
}
