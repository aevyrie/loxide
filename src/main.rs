use interpreter::Interpretation;
use std::error::Error;
use std::io::{self, prelude::*};
use std::path::Path;

mod interpreter;
mod parser;
mod scanner;

fn main() {
    let mut args = std::env::args();
    match {
        if args.len() > 2 {
            Err(UsageError::TooManyArgs.into())
        } else if let Some(arg) = args.nth(1) {
            run_file(Path::new(&arg))
        } else {
            run_prompt()
        }
    } {
        Ok(_) => println!("Goodbye!"),
        Err(error) => println!("{}", error),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UsageError {
    #[error("Usage: loxide [script]")]
    TooManyArgs,
}

fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let source = std::fs::read_to_string(path)?;
    run(source)
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    println!("loxide");
    print!("> ");
    io::stdout().flush()?;
    for line in lines {
        let buffer = line?;
        if let Err(error) = run(buffer) {
            println!("Error: {}", error);
        }
        print!("> ");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}

fn run(source: String) -> Result<(), Box<dyn Error>> {
    match scanner::scan(&source) {
        Ok(mut tokens) => {
            for token in tokens.iter() {
                print!("{}", token);
            }
            println!();
            match parser::parse(&mut tokens) {
                Ok(expr) => {
                    let val: String = expr.interpret();
                    dbg!(val);
                }
                Err(e) => panic!("failed to parse"),
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
