use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub struct IdentifierNode<'a> {
    pub token: Token<'a>,
}

impl<'a> IdentifierNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}
