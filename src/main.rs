use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Unwrap or unwrap_or_else is used when function returns.
    // unwrap return panics with a default message
    // using .expect let us choose the panic mesage instead of default. so
    // Result you unwrap the value.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
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

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // using '?' tells rust to return the error if theres one
    // also used when a Result is returned.
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents){
        println!("{line}");
    }
    Ok(())
}
