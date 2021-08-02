use scanner::Scanner;
use std::error::Error;
use std::io::{self, prelude::*};
use thiserror;

mod scanner;
mod token;

fn main() {
    let mut args = std::env::args();
    let result = {
        args.next();
        if args.len() > 1 {
            dbg!(args);
            Err(UsageError::TooManyArgs.into())
        } else if let Some(arg) = args.next() {
            run_file(arg)
        } else {
            run_prompt()
        }
    };
    if let Err(error) = result {
        println!("{}", error);
    } else {
        println!("Goodbye!");
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UsageError {
    #[error("Usage: loxide [script]")]
    TooManyArgs,
}

fn run_file(input_path: String) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(&input_path);
    let source = std::fs::read_to_string(path)?;
    run(source)
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("loxide");
    print!("> ");
    io::stdout().flush().unwrap();
    while let Some(line) = lines.next() {
        let buffer = line?;
        if let Err(error) = run(buffer) {
            println!("Error: {}", error);
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
    println!("");
    Ok(())
}

fn run(source: String) -> Result<(), Box<dyn Error>> {
    let scanner = Scanner::new(source);

    match scanner.tokens() {
        Ok(tokens) => {
            for token in tokens.iter() {
                dbg!(&token);
            }
        }
        Err(scan_errors) => {
            for error in scan_errors.iter() {
                println!("{}", error);
            }
        }
    }
    Ok(())
}
