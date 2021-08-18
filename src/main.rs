use colored::*;
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, getpid, ForkResult};
use std::ffi::CString;
use std::io::*;
use std::process::exit;
use std::vec::Vec;

fn main() {
    loop {
        print!("{}", "coconush >>>".green().bold());
        // At this point, release the buffer and output the prompt to the standard output
        match stdout().flush() {
            Ok(_) => {}
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        // Read input line
        let mut input_line = String::new();
        match stdin().read_line(&mut input_line) {
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

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                match waitpid(child, None) {
                    Ok(_) => {
                        
                    }
                    Err(_) => {
                        println!("Waitpid failed.");
                    }
                }
            }
            Ok(ForkResult::Child) => {
                execvp(&bin, &[&bin, &args]).expect("coconush error: failed exec.");
                exit(0)
            }
            Err(_) => {
                panic!("Fork failed.");
            }
        };
    }
}
