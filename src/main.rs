use std::{io, usize};
use std::io::Write;

fn main() {
    let mut list: Vec<String> = Vec::new();
    
    loop {    
        print!("todo> ");
        let mut command: String = String::new();
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let result = parse_cmd(&command);
        let (cmd, arg) = match result {
            Some((cmd, arg)) => (cmd, arg),
            None => {
                println!("error> invalid command!");
                continue;
            }
        };

        match cmd {
            "add" => list.push(arg.to_string()),
            "list" => {
                print_vec(&list);
            },
            "remove" => {
                let index = match arg.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        print!("error> Must be number");
                        continue;
                    }
                };
                list.remove(index);
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

fn parse_cmd(cmd: &str) -> Option<(&str, &str)> {
    let cmd= cmd.trim();
    if cmd == "list" {
        return Some((cmd, ""));
    }
    cmd.split_once(' ')
        .map(|(c, a)| (c.trim(), a.trim()))
}

fn print_vec(vec: &Vec<String>) {
    for (i, item) in vec.iter().enumerate() {
        println!("{i}) {item}");
    }
}