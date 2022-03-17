use super::parser::parse_file;
use crate::lisp::LanguageType;
use anyhow::Result;
use std::collections::HashMap;

pub fn interpret(file_contents: &str) -> Result<(&str, Vec<LanguageType>)> {
    let (input, result) = parse_file(file_contents).unwrap();
    Ok((input, result))
}

struct Scope {
    name: String,
    locals: HashMap<String, LanguageType>,
    parent: Box<Scope>,
    children: Vec<Scope>,
}
