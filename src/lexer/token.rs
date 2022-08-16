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

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("break_if")]
    BreakIf,

    #[token("in")]
    In,

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

    #[regex(r"[0-9]+\...[0-9]+")]
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
    pub line: usize,
    pub token_type: TokenType,
    pub slice: &'a str,
    pub span: Span,
}
