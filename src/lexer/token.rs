use core::fmt;
use std::fmt::{Display, Formatter};

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Binary operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    // Numbers
    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"[0-9]+\.[0-9]+")]
    Float,

    // Functions
    #[token("fn")]
    FunctionDecl,

    #[token("return")]
    Return,

    #[token("return_if")]
    ReturnIf,

    // Parenthesis
    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("{")]
    BlockStart,

    #[token("}")]
    BlockEnd,

    // Comments
    #[regex(r"//[^\n]*")]
    Comment,

    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")]
    CommentBlock,

    #[token(";")]
    Semicolon,

    // Whitespace
    #[token("\r")]
    CarriageReturn,

    #[token("\n")]
    LineFeed,

    #[token(" ")]
    Space,

    #[token("\t")]
    Tab,

    #[token("\0")]
    Null,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'a> {
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType,
    pub slice: &'a str,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(
        line: usize,
        column: usize,
        token_type: Result<TokenType, ()>,
        slice: &'a str,
        span: Span,
    ) -> Self {
        println!("{:?}", token_type.clone());
        Self {
            line,
            column,
            token_type: token_type.unwrap(),
            slice,
            span,
        }
    }
}
