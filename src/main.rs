mod prompt;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    loop {
        match prompt::display_prompt() {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
                exit(0);
            }
        };
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
                exit(0);
            }
        };

        // input command
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
                exit(0);
            }
        }
        line.remove(line.len() - 1);

        // debug
        println!("{}", line);
    }
}
