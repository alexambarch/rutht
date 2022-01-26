use anyhow::Result;
use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited, IResult};

pub fn interpret(file_contents: &str) -> Result<(&str, &str)> {
    let result = sexp(&file_contents).unwrap();
    Ok(result)
}

fn sexp(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}
