use std::io::*;

fn main() {
  print!("Input your command >>>");
  match stdout().flush() {
    Ok(_) => {}
    Err(error) => {
      eprintln!("{}", error);
    }
  }
  
  let mut input_line = String::new();
  match stdin().read_line(&mut input_line) {
    Ok(_) => {
      println!("Input: {}", &input_line);
    }
    Err(error) => {
        println!("Input error: {}", error);
    }
  }
}