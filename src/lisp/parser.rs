use crate::lib::string_parser::parse_string;
use crate::util::Literal;
use nom::{
    branch::alt, character::complete::{char, digit1}, sequence::preceded, IResult, multi::many0,
};

/// Parse a (possibly negative) number
fn parse_number(input: &str) -> IResult<&str, Literal> {
    let (input, num_text) = preceded(many0(char('-')), digit1)(input)?;
    Ok((input, Literal::Number(num_text.parse::<i64>().unwrap())))
}

/// Parse a literal value, either a string or a number
fn parse_literal(input: &str) -> IResult<&str, Literal> {
    alt((parse_string, parse_number))(input)
}
