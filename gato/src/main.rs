use owo_colors::OwoColorize;
use std::env::args;
use std::error::Error;
use std::fs;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_line: Vec<String> = args().collect();
    let line: Vec<&str> = raw_line.iter().map(|x| x.as_str()).collect();
    if line.len() < 2 {
        eprint!("no file specified");
        exit(1)
    }
    let fline = &line[1..];
    let _ = gatito(fline);
    Ok(())
}

fn gatito(files: &[&str]) -> Result<(), Box<dyn Error>> {
    for file in files {
        let cont_res = fs::read_to_string(file);
        let cont = match cont_res {
            Ok(file) => file,
            Err(_e) => {
                eprintln!("{}", "Problem reading file".red().bold());
                exit(1)
            }
        };
        println!("{}", &cont);
    }
    Ok(())
}
