use std::io;

fn main() {
    // Read input line
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Shelly: Input error.");

    // Parse input line
    let command: Vec<&str> = input_line.split_whitespace().collect();
    
    for argument in command {
        println!("{:?}", argument);
    }
    
}