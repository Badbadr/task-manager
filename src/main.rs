use std::{io, usize};
use std::io::Write;

fn main() {
    println!("type `help` for commands list");
    let mut list: Vec<String> = Vec::new();
    
    loop {    
        print!("todo> ");
        let mut command: String = String::new();
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        // parse_cmd(&command);
        let result = command.trim()
            .split_once(' ')
            .or(Some((&command, "")))
            .map(|(c, a)| (c.trim(), a.trim()));

        let (cmd, arg) = match result {
            Some((cmd, arg)) => (cmd, arg),
            None => {
                println!("error> invalid command!");
                continue;
            }
        };

        match cmd {
            "add" => list.push(arg.to_string()),
            "list" => print_vec(&list),
            "help" => print_help(), 
            "remove" => {
                let index = match arg.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        print!("error> Must be number");
                        continue;
                    }
                };
                match list.get(index) {
                    Some(_) => list.remove(index),
                    None => {
                        println!("error> index out of bounds");
                        continue;
                    }
                };
            },
            "done" => {
                let index: usize = match arg.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("error> Must be number");
                        continue;
                    }
                };
                let arg= match list.get_mut(index) {
                    Some(item) => item,
                    None => {
                        println!("error> invalid index");
                        continue;
                    }
                };
                arg.insert_str(0, "✅");
            },
            _ => {
                println!("todo> unknown command");
            }
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