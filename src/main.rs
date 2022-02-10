use clap::Parser;
use std::fs::read_to_string;

mod lib;
mod lisp;

#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret.
    file: String,
}

fn main() {
    let args = Args::parse();

    let contents = read_to_string(args.file).unwrap();

    let (_, lisp) = lisp::interpreter::interpret(&contents).unwrap();

    println!("{:?}", lisp);
}
