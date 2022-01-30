use clap::Parser;
use std::fs::read_to_string;

mod lib;
mod lisp;
mod util;

#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret.
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut contents = read_to_string(args.file).unwrap();

    let result = lisp::interpreter::interpret(&contents).unwrap();

    println!("{}, {}", result.0, result.1);
}
