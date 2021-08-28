use std::env;
use std::io::Result;
use whoami::{hostname, username};

pub fn display_prompt() -> Result<()> {
    let current_path = env::current_dir()?;
    print!(
        "{} @{} :{} >",
        username(),
        hostname(),
        current_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_current_directory() {
        assert!(display_prompt().is_ok());
    }
}
