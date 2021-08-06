use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, getpid, getppid, ForkResult};
use std::ffi::CString;
use std::io;
use std::process::exit;
use std::vec::Vec;

fn main() {
    loop {
        // Read input line
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(_) => {}
            Err(error) => {
                println!("coconush error: {}", error);
            }
        }

        // Parse input line
        // "foo bar baz" => ["foo", "bar", "baz"]
        let command: Vec<&str> = input_line.split_whitespace().collect();
        let bin = CString::new(command[0].to_string()).unwrap();
        let args = CString::new(command[1].to_string()).unwrap();

        for term in command {
            println!("{:?}", term);
        }

        println!("Current process id: {}", getpid());

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                println!("Main({}) forked a child({})", getpid(), child);
                match waitpid(child, None) {
                    Ok(_pid) => {
                        println!("Child exited {:?}.", child);
                    }
                    Err(_) => {
                        println!("Waitpid failed.");
                    }
                }
            }
            Ok(ForkResult::Child) => {
                println!("Child({}) started. PPID is {}", getpid(), getppid());
                execvp(&bin, &[&bin, &args]).expect("coconush error: failed exec.");
                exit(0)
            }
            Err(_) => {
                panic!("Fork failed.");
            }
        };
    }
}
