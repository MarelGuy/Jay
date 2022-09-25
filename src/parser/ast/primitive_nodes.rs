use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub struct NumberNode<'a> {
    token: Token<'a>,
}

impl<'a> NumberNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierNode<'a> {
    token: Token<'a>,
}

impl<'a> IdentifierNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}
