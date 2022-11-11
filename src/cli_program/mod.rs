// cargo run -- searchstring example-filename.txt

mod models;
mod tests;

use std::{env, process};
use std::error::Error;
use models::config::Config;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_command(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

/// Box<dyn Error> means return a type that implements Error but we don't what type it is
fn run_command(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(&config.file_path)?;
    println!("{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
