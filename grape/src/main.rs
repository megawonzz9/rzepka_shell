use colored::*;
use owo_colors::OwoColorize;
use regex::Regex;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_line: Vec<String> = args().collect();
    let mut line: Vec<&str> = raw_line.iter().map(|x| x.as_str()).collect();
    if line.len() < 2 {
        eprint!("yo need atleast 2 args");
        exit(1);
    }
    if let Some(word) = line.iter().find(|w| w.starts_with('-')) {
        match *word {
            "-f" => with_file(&line)?,
            "-i" => no_case(&line)?,
            "-s" => {
                line.retain(|&s| s != "-s");
                strict(&line)?
            }
            _ => println!("not known flag"),
        }
    } else {
        let _ = piped(&line);
    }

    Ok(())
}

fn with_file(line: &[&str]) -> Result<(), Box<dyn Error>> {
    let line_f = &line[1..];
    if let Some(filez) = line_f.iter().position(|&minus| minus == "-f") {
        let del_args = &line_f[filez..filez + 2];
        let args: Vec<&str> = line_f
            .iter()
            .filter(|&&x| !del_args.contains(&x))
            .copied()
            .collect();
        let filename = line_f[filez + 1];
        let filef = File::open(filename)?;
        let reader = BufReader::new(filef);
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if args.iter().all(|arg| line.contains(arg)) {
                let (result, matched) = search_optimized(&line, &args, None);
                if matched {
                    println!("{}: {}", line_num.cyan(), result);
                }
            }
        }
    } else {
        eprintln!("bad use o -f")
    }

    Ok(())
}

fn piped(line: &[&str]) -> Result<(), Box<dyn Error>> {
    let keywords = &line[1..];
    for (line_num, line) in stdin().lock().lines().enumerate() {
        let line = line?;
        if keywords.iter().any(|kw| line.contains(kw)) {
            let (result, matched) = search_optimized(&line, keywords, None);
            if matched {
                println!("{}: {}", line_num.cyan(), result);
            }
        }
    }
    Ok(())
}

fn no_case(line: &[&str]) -> Result<(), Box<dyn Error>> {
    let keywords = &line[1..];
    for (line_num, line) in stdin().lock().lines().enumerate() {
        let line = line?;
        let line_low = line.to_lowercase();
        if keywords
            .iter()
            .any(|kw| line_low.contains(&kw.to_lowercase()))
        {
            let (result, matched) = search_optimized(&line, keywords, Some(1));
            if matched {
                println!("{}: {}", line_num.cyan(), result);
            }
        }
    }
    Ok(())
}

fn search_optimized(line: &str, patterns: &[&str], opts: Option<u8>) -> (String, bool) {
    let opts = opts.unwrap_or(0);
    let joined_patterns = patterns.join("|"); // Combine all patterns
    let regex = match opts {
        1 => Regex::new(&format!(r"(?i){}", joined_patterns)).unwrap(),
        2 => Regex::new(&format!(r"(?i)\b({})\b", joined_patterns)).unwrap(),
        _ => Regex::new(&joined_patterns).unwrap(),
    };

    let replaced = regex
        .replace_all(line, |caps: &regex::Captures| {
            caps[0].red().bold().to_string()
        })
        .to_string();

    (replaced.clone(), replaced != line)
}

fn strict(line: &[&str]) -> Result<(), Box<dyn Error>> {
    let keywords = &line[1..];
    for (line_num, line) in stdin().lock().lines().enumerate() {
        let line = line?;
        if keywords.iter().all(|kw| line.contains(kw)) {
            let (result, matched) = search_optimized(&line, keywords, Some(2));
            if matched {
                println!("{}: {}", line_num, result);
            }
        }
    }
    Ok(())
}
