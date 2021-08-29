mod prompt;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        // display status
        match prompt::display_prompt() {
            Ok(_) => {}
            Err(e) => {
                println!("prompt error: {}", e);
            }
        }
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                println!("buf error: {}", e);
            }
        }

        // input line
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                println!("read line error: {}", e);
            }
        }

        // parse input
        let mut parts = line.trim().split_whitespace();
        let command = parts.next().unwrap_or("\n");
        let args = parts;

        // exec command
        match Command::new(command).args(args).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(_) => {}
                Err(e) => {
                    println!("wait error: {}", e);
                }
            },
            Err(e) => {
                println!("exec error: {}", e);
            }
        }
    }
}
