pub enum LanguageType {
    Symbol(String),
    LiteralValue(Literal),
}

pub enum Literal {
    Number(i64),
    String(String),
}
