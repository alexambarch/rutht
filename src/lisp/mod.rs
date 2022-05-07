pub mod interpreter;
pub mod parser;

#[derive(Debug)]
pub enum LanguageType {
    Symbol(String),
    Literal(Literal),
    Collection(Vec<LanguageType>),
    ArgList(Vec<LanguageType>),
    Function {
        name: Box<LanguageType>,
        args: Box<LanguageType>,
    },
    Nil,
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(String),
}

// I'm taking all of your RAM and you're gonna like it
#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
