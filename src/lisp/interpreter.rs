use anyhow::Result;
use crate::util::LanguageType;
use super::parser::parse_funcall;

pub fn interpret(file_contents: &str) -> Result<(&str, LanguageType)> {
    let (input, result) = parse_funcall(file_contents).unwrap();
    Ok((input, result))
}
