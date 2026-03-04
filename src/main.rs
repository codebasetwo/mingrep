use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Unwrap or unwrap_or_else is used when function returns.
    // unwrap return panics with a default message
    // using .expect let us choose the panic mesage instead of default. so
    // Result you unwrap the value.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Instead of panicking we can retun result
            // panic("Not enough arguments.");
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // We don’t care about the value of the environment variable,
        // just whether it’s set or unset, 'is_ok()'
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // using '?' tells rust to return the error if theres one
    // also used when a Result is returned.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
