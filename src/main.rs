use std::io;

fn main() {
    // Read input line
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Shelly: Input error.");

    // Parse input line
    // "foo bar baz" => ["foo", "bar", baz]
    let command: Vec<&str> = input_line.split_whitespace().collect();
    
    // Print debug for vector
    for argument in command {
        println!("{:?}", argument);
    }
    
}