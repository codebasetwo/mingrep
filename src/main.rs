use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1]; // file path
    let query = &args[2]; // word to search

    println!("{query} {file_path}");

    // Reaf content of the file
    let contents = fs::read_to_string(file_path)
        .expect("should have read in a file.");

    println!("With text:\n{contents}");
}
