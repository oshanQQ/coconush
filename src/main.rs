mod prompt;
use std::io::Result;
use std::io::{stdin, stdout, Write};

fn main() -> Result<()> {
    loop {
        prompt::display_prompt()?;
        stdout().flush()?;

        // input command
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        line.remove(line.len() - 1);

        // debug
        println!("{}", line);
    }
}
