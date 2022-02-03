use crate::lib::string_parser::parse_string;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{opt, recognize},
    multi::{fold_many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

/// Parse a (possibly negative) number
fn parse_number(input: &str) -> IResult<&str, &str> {
    recognize(tuple((opt(char('-')), digit1)))(input)
}

/// Parse a literal value, either a string or a number
fn parse_literal(input: &str) -> IResult<&str, &str> {
    alt((parse_string, parse_number))(input)
}

/// Parse a symbol/identifier
fn parse_symbol(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

/// Parse a bunch of heterogenous values
fn parse_many_vals(input: &str) -> IResult<&str, &str> {
    recognize(separated_list0(
        char(' '),
        alt((parse_number, parse_string, parse_symbol)),
    ))(input)
}

/// Parse heterogenous collection of values inside of a collection
fn parse_collection(input: &str) -> IResult<&str, &str> {
    delimited(char('['), parse_many_vals, char(']'))(input)
}

/// Parse a function call
fn parse_funcall(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('('),
        parse_symbol,
        opt(parse_collection),
        char(')'),
    )))(input)
}
