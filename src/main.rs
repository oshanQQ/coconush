mod prompt;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
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
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("cd error: {}", e);
                }
            }
            "exit" => return,
            command => match Command::new(command).args(args).spawn() {
                Ok(mut child) => {
                    if let Err(e) = child.wait() {
                        eprintln!("wait error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("exec error: {}", e);
                }
            },
        }
    }
}
