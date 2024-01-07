use std::{io, collections::HashMap};

#[derive(Debug)]
enum ListingType {
    All,
    Department { department_name: String }
}

#[derive(Debug)]
enum Command {
    Help,
    Exit,
    Add { name: String, department: String },
    List { listing_type: ListingType },
    Unknown
}

impl Command {
    fn parse_command(command_line: &str) -> Command {
        let command_line_lowercase = command_line.trim().to_lowercase();
        if command_line_lowercase == "help" {
            Command::Help
        } else if command_line_lowercase == "exit" {
            Command::Exit
        } else if command_line_lowercase.starts_with("list") {
            let list_command_split: Vec<&str> = command_line.split_whitespace().collect();
            if list_command_split.len() == 2 {
                if list_command_split[1].to_lowercase() == "all" {
                    Command::List { listing_type: ListingType::All }
                } else {
                    Command::List { listing_type: ListingType::Department { department_name: list_command_split[1].to_string() } }
                }
            } else {
                Command::Unknown
            }
        } else if command_line_lowercase.starts_with("add") {
            let add_command_split: Vec<&str> = command_line.split_whitespace().collect();
            if add_command_split.len() == 4 && add_command_split[2].to_lowercase() == "to" && add_command_split[2].to_lowercase() != "all" {
                Command::Add { name: add_command_split[1].to_string(), department: add_command_split[3].to_string() }
            } else {
                Command::Unknown
            }
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
            Command::Help => {
                println!("Use `exit` to exit the program");
                println!("Use `add X to Y` to add person `X` to department `Y`");
                println!("Use `list ALL` to show all people in the company and `list X` to show people in department `X`");
            },
            Command::List { listing_type: ListingType::All } => println!("The people working in the company: {:#?}", database),
            Command::List { listing_type: ListingType::Department { department_name } } => {
                match database.get(&department_name) {
                    Some(people) => println!("The people working in {department_name}: {:#?}", people),
                    None => println!("Whoops, seems no one's working in department {department_name}")
                }
            },
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
