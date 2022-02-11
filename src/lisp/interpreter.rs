use anyhow::Result;
use crate::lisp::LanguageType;
use super::parser::parse_file;

pub fn interpret(file_contents: &str) -> Result<(&str, Vec<LanguageType>)> {
    let (input, result) = parse_file(file_contents).unwrap();
    Ok((input, result))
}
