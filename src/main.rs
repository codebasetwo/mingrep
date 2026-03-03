use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_args(&args);

    println!("{query} {file_path}");

    // Reaf content of the file
    let contents = fs::read_to_string(file_path)
        .expect("should have read in a file.");

    println!("With text:\n{contents}");
}

fn parse_args(args: &[String]) -> (&String, &String) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}