/*
Jay tokens
Copyright (C) 2022  Loris Cuntreri
*/

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum TokenType {
    // Math operators
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

    // Logic operators
    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    // Comparison operators
    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    // Assignment operators
    #[token("=")]
    Assign,

    #[token("+=")]
    PlusAssign,

    #[token("-=")]
    MinusAssign,

    #[token("*=")]
    MultiplyAssign,

    #[token("/=")]
    DivideAssign,

    #[token("%=")]
    ModuloAssign,

    // Delimiters
    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    // Types
    #[token("int")]
    IntType,

    #[token("float")]
    FloatType,

    #[token("bool")]
    BoolType,

    #[token("string")]
    StringType,

    #[token("void")]
    VoidType,

    #[token("char")]
    CharType,

    // Keywords
    #[token("type")]
    Type,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("return")]
    Return,

    #[token("func")]
    Func,

    #[token("let")]
    Let,

    #[token("var")]
    Var,

    #[token("const")]
    Const,

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Numbers
    #[regex(r"[0-9]+")]
    Number,

    // Strings
    #[regex(r#""[^"]*""#)]
    String,

    // Punctuation
    #[token(".")]
    Dot,

    #[token("...")]
    Ellipsis,

    #[token(":")]
    Colon,

    // Comments
    #[regex(r"//[^\n]*")]
    Comment,

    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")]
    BlockComment,

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

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Token<'source> {
    pub token_type: TokenType,
    pub slice: &'source str,
    pub span: Span,
}
