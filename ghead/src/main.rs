use atty::Stream;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

fn main() {
    // Collect command-line arguments
    let raw_line: Vec<String> = args().collect();
    let mut line: Vec<&str> = raw_line.iter().map(|x| x.as_str()).collect();
    line.remove(0); // Remove the program name from arguments

    // Default number of lines
    let mut num_lines = 10;

    // Parse `-n` flag and number of lines
    if let Some(pos) = line.iter().position(|&arg| arg == "-n") {
        if pos + 1 < line.len() {
            num_lines = line[pos + 1]
                .parse::<usize>()
                .expect("Error: -n flag must be followed by a number");
            line.drain(pos..=pos + 1); // Remove `-n` and its value from arguments
        } else {
            eprintln!("Error: -n flag must be followed by a number");
            exit(1);
        }
    }

    // Check for piped input
    if atty::is(Stream::Stdin) {
        // If no piped input, check for file input
        if let Some(&file_path) = line.get(0) {
            // Read lines from the specified file
            if let Err(err) = read_file(file_path, num_lines) {
                eprintln!("Error reading file: {}", err);
                exit(1);
            }
        } else {
            // No file specified and no piped input
            eprintln!("Usage: [PROGRAM] -n <number_of_lines> [file]");
            exit(1);
        }
    } else {
        // Handle piped input
        if let Err(err) = piped(num_lines) {
            eprintln!("Error reading stdin: {}", err);
            exit(1);
        }
    }
}

fn read_file(file_path: &str, lines: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == lines {
            break;
        }
        println!("{}", line);
    }

    Ok(())
}

fn piped(lines: usize) -> Result<(), Box<dyn Error>> {
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        if line_num == lines {
            break;
        }
        println!("{}", line);
    }
    Ok(())
}
