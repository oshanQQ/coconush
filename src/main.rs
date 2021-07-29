use std::io;

fn main() {
    let mut input_line = String::new();
    match io::stdin().read_line(&mut input_line) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("Input: {}", &input_line);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}