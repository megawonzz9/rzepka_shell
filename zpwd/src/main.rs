use owo_colors::OwoColorize;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let working_dir = env::current_dir()?;
    println!("{}", working_dir.display().purple());
    Ok(())
}
