use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub struct PrimitiveTypeNode<'a> {
    token: Token<'a>,
}

impl<'a> PrimitiveTypeNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}
