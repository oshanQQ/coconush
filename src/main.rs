use std::io;
use std::process::Command;

fn main() {
    // Read input line
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Shelly: Input error.");

    // Parse input line
    // "foo bar baz" => ["foo", "bar", "baz"]
    let command: Vec<&str> = input_line.split_whitespace().collect();

    Command::new(&command[0])
        .arg(command[1])
        .spawn()
        .unwrap();
    
}