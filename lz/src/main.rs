use std::{error::Error, fs::read_dir};

fn main() -> Result<(), Box<dyn Error>>{

    let entries = read_dir(".");
    for entry in entries {
        let entry = entry?;
        let path = entry.path();


        // Get metadata to determine entry type
        let metadata = entry.metadata()?;
        
        if metadata.is_file() {
            println!("File: {}", path.display());
        } else if metadata.is_dir() {
            println!("Directory: {}", path.display());

    }
}
