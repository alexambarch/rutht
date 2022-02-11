use super::parser::parse_file;
use crate::lisp::LanguageType;
use anyhow::Result;

pub fn interpret(file_contents: &str) -> Result<(&str, Vec<LanguageType>)> {
    let (input, result) = parse_file(file_contents).unwrap();
    Ok((input, result))
}
