use std::io;
use nix::unistd::{fork, getpid, getppid, ForkResult};

fn main() {
    // Read input line
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Shelly: Input error.");

    // Parse input line
    // "foo bar baz" => ["foo", "bar", "baz"]
    let command: Vec<&str> = input_line.split_whitespace().collect();

    for term in command {
        println!("{:?}", term);
    }

    println!("Current process id: {}", getpid());

    unsafe {
        match fork() {
            Ok(ForkResult::Parent {child}) => {
                println!("Main({}) forked a child({})", getpid(), child);
            }
            Ok(ForkResult::Child) => {
                println!("Child({}) started. PPID is {}", getpid(), getppid());
            }
            Err(_) => {
                println!("Fork failed.");
            }
        }
    }
}