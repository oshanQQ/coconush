mod prompt;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        // display status
        match prompt::display_prompt() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("prompt error: {}", e);
            }
        }
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("buf error: {}", e);
            }
        }

        // input line
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("read line error: {}", e);
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
                    eprintln!("wait error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("exec error: {}", e);
            }
        }
    }
}
