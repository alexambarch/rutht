mod lisp;

use clap::Parser;
use lisp::interpreter::interpret;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret.
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut contents = read_to_string(args.file).unwrap();
    contents.retain(|c| c != '\n');

    let result = interpret(&contents).unwrap();

    println!("{}, {}", result.0, result.1);
}
