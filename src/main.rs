#![allow(unused)]

use std::{ env::args, path::PathBuf, io::BufReader, task::Context };
use clap::Parser;

// Search for a pattern in a given file and display the lines that contain it.
#[derive(Parser)]
struct CLI {
    // The pattern to look for
    pattern: String,

    // The path to the file to read
    path: PathBuf,
}

fn main() {
    let args: CLI = CLI::parse();

    let content: String = std::fs::read_to_string(&args.path).expect("Couldn't read file");

    let result = std::fs::read_to_string("test.txt");
    match result {
        Ok(content) => {
            content;
        }
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    }
    println!("file content: {}", content);

    // This looks for the pattern in the given file at the given path
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
