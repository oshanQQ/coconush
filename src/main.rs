mod prompt;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        // display status
        if let Err(e) = prompt::display_prompt() {
            eprintln!("prompt error: {}", e);
        }
        if let Err(e) = stdout().flush() {
            eprintln!("buf error: {}", e);
        }

        // input line
        let mut line = String::new();
        if let Err(e) = stdin().read_line(&mut line) {
            eprintln!("read line error: {}", e);
        }

        // parse input
        let mut parts = line.trim().split_whitespace();
        let command = parts.next().unwrap_or("\n");
        let args = parts;

        // exec command

        match Command::new(command).args(args).spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprintln!("wait error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("exec error: {}", e);
            }
        }
    }
}
