use std::io;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult};
use std::process::exit;

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
        let pid = match fork() {
            Ok(ForkResult::Parent {child}) => {
                // I'm a parent process.
                println!("Main({}) forked a child({})", getpid(), child);
                child
            }
            Ok(ForkResult::Child) => {
                // I'm a child process.
                println!("Child({}) started. PPID is {}", getpid(), getppid());
                exit(0)
            }
            Err(_) => {
                panic!("Fork failed.");
            }
        };

        match waitpid(pid, None) {
            Ok(status) => {
                println!("Child exited {:?}.", status);
            }
            Err(_) => {
                println!("Waitpid failed.");
            }
        }
    }
}