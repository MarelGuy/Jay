use crate::lexer::token::{Span, Token, TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_token: Token<'a>,
}

impl Parser<'_> {
    pub fn new(token_stream: Vec<Token>) -> Parser {
        Parser {
            tokens: token_stream,
            current_token: Token {
                token_type: TokenType::Null,
                slice: "0",
                span: Span { start: 0, end: 0 },
            },
        }
    }
    pub fn parse(&mut self) {}
}
