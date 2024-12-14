use anyhow::Result;
use std::io::{self, BufRead, Write};

mod argparser;
mod scanner;
mod utils;

use argparser::argparser::parse_args;
use scanner::scanner::run;
use utils::read_file::read_file;

fn run_file(file: &str) {
    let contents = read_file(file).expect("Error reading file");
    println!("Running file: {}", file);
    run(contents);
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?; // Ensure the prompt is displayed immediately

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break; // EOF reached
        }

        run(line.trim().to_string());
    }

    Ok(())
}

fn main() {
    let args = parse_args();
    match args.script {
        Some(file) => run_file(&file),
        None => run_prompt().expect("Error running prompt"),
    }
}
