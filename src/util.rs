#[derive(Debug)]
pub enum LanguageType {
    Symbol(String),
    LiteralValue(Literal),
}

#[derive(Debug)]
pub enum Literal {
    Number(i64),
    String(String),
}
