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

    let contents = read_to_string(args.file).unwrap();

    let (_, (function, args)) = lisp::interpreter::interpret(&contents).unwrap();

    println!("{:?}, {:?}", function, args);
}
