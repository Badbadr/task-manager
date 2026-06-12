use std::{io, usize, fmt};
use std::io::Write;

struct Task {
    name: String,
    done: bool
}

// TODO: research
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.done {
            write!(f, "✅ {}", self.name)
        }  else {
            write!(f, "{}", self.name)
        }
    }
}

impl Task {
    fn new(name: String) -> Self {
        Task{name: name, done: false}
    }
}

enum Command {
    Add { arg: String },
    Remove { arg: usize},
    Done { arg: usize },
    List,
    Help,
    Exit
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

        match cmd {
            "add" => Some(Command::Add{ arg: arg.to_string() }),
            "list" => Some(Command::List),
            "help" => Some(Command::Help),
            "exit" => Some(Command::Exit),
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
        }
    }

    fn execute(self, tasks: &mut Vec<Task>) {
        match self {
            Command::Help => {print_help()}
            Command::List => {print_vec(tasks)},
            Command::Exit => (),
            Command::Add { arg} => tasks.push(Task::new(arg)),
            Command::Done { arg } => {
                let task = match tasks.get_mut(arg) {
                    Some(s) => s,
                    None => {
                        println!("error> index out of bounds");
                        return
                    }
                };
                task.done = true;
                
            },
            Command::Remove { arg } => {
                match tasks.get(arg) {
                    Some(_) => tasks.remove(arg),
                    None => {
                        println!("error> index out of bounds");
                        return
                    }
                };
            },

        }
    }
}

fn main() {
    println!("type `help` for commands list");
    let mut tasks: Vec<Task> = Vec::new();
    
    loop {    
        print!("todo> ");
        io::stdout().flush().unwrap();
        let cmd = Command::new_from_input();
        match cmd {
            Some(Command::Exit) => break,
            Some(command) => command.execute(&mut tasks),
            None => ()   
        }
    }
}

fn print_help() {
    println!("
        Welcome to task manager! It's quite simple yet.
        Available commands are:
        - add <arg> –– add task to list
        - done <num> –– mark task as done
        - remove <num> –– remove task from list
        - help –– for information
        - exit –– close application
    ");
}

fn print_vec(vec: &[Task]) {
    for (i, item) in vec.iter().enumerate() {
        println!("{i}) {item}");
    }
}