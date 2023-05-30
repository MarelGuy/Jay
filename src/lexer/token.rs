use core::fmt;
use std::fmt::{Display, Formatter};

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

    // Logic operators
    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("|")]
    Next,

    // Comparison operators
    #[token("==")]
    Equal,

    #[token("===")]
    StrictEqual,

    #[token("!=")]
    NotEqual,

    #[token("!==")]
    NotStrictEqual,

    #[token("<")]
    LessThan,

    #[token("<<")]
    StrictLessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token("<=<")]
    StrictLessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">>")]
    StrictGreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token(">=>")]
    StrictGreaterThanOrEqual,

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

    #[token("^=")]
    PowerAssign,

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

    #[token("for")]
    For,

    #[token("loop")]
    Loop,

    #[token("return")]
    Return,

    #[token("return_if")]
    ReturnIf,

    #[token("func")]
    Func,

    #[token("let")]
    Let,

    #[token("var")]
    Var,

    #[token("const")]
    Const,

    #[regex(r"true|false")]
    Bool,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("break_if")]
    BreakIf,

    #[token("in")]
    In,

    #[token("pub")]
    Pub,

    #[token("priv")]
    Priv,

    // Switch
    #[token("switch")]
    Switch,

    #[token("case")]
    Case,

    #[token("default")]
    Default,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Numbers
    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"[0-9]+\.[0-9]+")]
    Float,

    #[regex(r"-[0-9]+")]
    NegativeNumber,

    #[regex(r"-[0-9]+\.[0-9]+")]
    NegativeFloat,

    #[regex(r"[0-9]+\.\.\.[0-9]+")]
    Range,

    // Strings and Chars
    #[regex(r#""[^"]*""#)]
    String,

    #[regex(r#"'.'"#)]
    Char,

    // Punctuation
    #[token(".")]
    Dot,

    #[token("...")]
    Ellipsis,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token(":::")]
    TripleColon,

    // Import and Export
    #[token("import")]
    Import,

    #[token("export")]
    Export,

    #[token("from")]
    From,

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
        Self {
            line,
            column,
            token_type: token_type.unwrap(),
            slice,
            span,
        }
    }
}
