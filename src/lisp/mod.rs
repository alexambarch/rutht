pub mod interpreter;
pub mod parser;

#[derive(Debug)]
pub enum LanguageType {
    Symbol(String),
    LiteralValue(Literal),
    ArgList(Vec<LanguageType>),
    Function { name: Box<LanguageType>, args: Box<LanguageType> },
    Nil,
}

#[derive(Debug)]
pub enum Literal {
    Number(i64),
    String(String),
}
