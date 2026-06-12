use std::{io, usize};
use std::io::Write;

enum Command {
    Add { arg: String },
    Remove { arg: usize},
    Done { arg: usize },
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
        let cmd_input = command.trim().split_once(' ')
            .or(Some((&command, "")));

        let (cmd, arg) = match cmd_input {
            Some((c, a)) => (c.trim(), a.trim()),
            None => {
                println!("error> invalid command!");
                return None;
            }
        };

        return match cmd {
            "add" => Some(Command::Add{ arg: arg.to_string() }),
            "list" => Some(Command::List),
            "help" => Some(Command::Help),
            "remove" => {
                let arg: usize = match arg.parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("error> Must be number");
                        return None
                    }
                };
                Some(Command::Remove { arg: arg })
            },
            "done" => {
                let arg = match arg.parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("error> Must be number");
                        return None
                    }
                };
                Some(Command::Done { arg: arg })
            },
            _ => {
                println!("error> unknown command!");
                None
            }
        };
    }

    fn execute(self, tasks: &mut Vec<String>) {
        match self {
            Command::Add { arg} => tasks.push(arg),
            Command::Done { arg } => {
                match tasks.get_mut(arg) {
                    Some(s) => s,
                    None => {
                        println!("error> index out of bounds");
                        return
                    }
                }.insert_str(0, "✅");
            },
            Command::List => {print_vec(tasks)},
            Command::Remove { arg } => {
                match tasks.get(arg) {
                    Some(_) => tasks.remove(arg),
                    None => {
                        println!("error> index out of bounds");
                        return
                    }
                };
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