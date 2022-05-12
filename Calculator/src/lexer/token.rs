use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Binary operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("^")]
    Power,

    #[token("%")]
    Modulo,

    // Unary operators
    #[token("++")]
    PlusPlus,

    #[token("--")]
    MinusMinus,

    // Numbers
    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"[0-9]+\.[0-9]+")]
    Float,

    // Spaces
    #[token(" ")]
    Space,

    // Null
    #[token("\0")]
    Null,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
    pub span: Span,
}
