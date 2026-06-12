use std::{io, usize};
use std::io::Write;

enum Command {
    Add { args: Vec<String> },
    Remove { args: Vec<String>},
    Done { args: Vec<String> },
    List,
    Help,
}

impl Command {

    fn new_from_input() -> Option<Self> {
        let mut command: String = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        // parse_cmd(&command);
        let mut cmd_input = command.trim().split_whitespace();

        let cmd = match cmd_input.next() {
            Some(str) => str,
            None => {
                println!("error> invalid command!");
                return None;
            }
        };

        let mut args: Vec<String> = Vec::new();
        cmd_input.for_each(|i| args.push(i.to_string()));

        return match cmd {
            "add" => Some(Command::Add { args }),
            "remove" => Some(Command::Remove { args }),
            "list" => Some(Command::List),
            "done" => Some(Command::Done { args }),
            "help" => Some(Command::Help),
            _ => {
                println!("error> unknown command!");
                None
            }
        };
    }

    fn execute(&self, tasks: &mut Vec<String>) {
        match self {
            Command::Add { args } => {
                let task = match args.get(0) {
                    Some(s) => s.to_string(),
                    None => {
                        print!("error> Invalid add command args");
                        return
                    }
                };
                tasks.push(task);
            },
            Command::Done { args } => {
                if let Some(s) = args.get(0) {
                    let index: usize = match s.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("error> Must be number");
                            return
                        }
                    };
                    let arg= match tasks.get_mut(index) {
                        Some(item) => item,
                        None => {
                            println!("error> invalid index");
                            return
                        }
                    };
                    arg.insert_str(0, "✅");
                } else {
                    println!("error> empty arguments list");
                }
            },
            Command::List => {print_vec(tasks)},
            Command::Remove { args } => {
                if let Some(s) = args.get(0) {
                    let index: usize = match s.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            print!("error> Must be number");
                            return
                        }
                    };
                    match tasks.get(index) {
                        Some(_) => tasks.remove(index),
                        None => {
                            println!("error> index out of bounds");
                            return
                        }
                    };
                } else {
                    println!("error> empty arguments list");
                }
            },
            Command::Help => {print_help()}
        }
    }
}

fn main() {
    println!("type `help` for commands list");
    let mut tasks: Vec<String> = Vec::new();
    
    loop {    
        print!("todo> ");
        io::stdout().flush().unwrap();
        let cmd = Command::new_from_input();
        match cmd {
            Some(command) => command.execute(&mut tasks),
            None => ()   
        }
    }
}

fn print_vec(vec: &Vec<String>) {
    for (i, item) in vec.iter().enumerate() {
        println!("{i}) {item}");
    }
}

fn print_help() {
    println!("
        Welcome to task manager! It's quite simple yet.\n
        Available commands are:\n
        - add <arg> –– add task to list\n
        - done <num> –– mark task as done\n
        - remove <num> –– remove task from list\n
        - help –– for information\n
    ");
}