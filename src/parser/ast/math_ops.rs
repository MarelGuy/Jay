use either::Either;

use crate::lexer::token::{Span, Token, TokenType};

use super::{identifier::IdentifierNode, types::NumberNode};

#[derive(PartialEq, Debug, Clone)]
pub struct OpNode<'a> {
    operation: Either<BinOpNode<'a>, UnOpNode<'a>>,
}

impl<'a> OpNode<'a> {
    pub fn new(operation: Either<BinOpNode<'a>, UnOpNode<'a>>) -> Self {
        Self { operation }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BinOpNode<'a> {
    pub left_node: Either<Either<NumberNode<'a>, IdentifierNode<'a>>, Box<BinOpNode<'a>>>,
    pub op_token: Token<'a>,
    pub right_node: Either<NumberNode<'a>, IdentifierNode<'a>>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(
        left_node: Either<Either<NumberNode<'a>, IdentifierNode<'a>>, Box<BinOpNode<'a>>>,
        op_token: Token<'a>,
        right_node: Either<NumberNode<'a>, IdentifierNode<'a>>,
    ) -> Self {
        Self {
            left_node,
            op_token,
            right_node,
        }
    }

    pub fn new_useless(line: usize, column: usize, span: Span) -> Self {
        Self {
            left_node: Either::Left(Either::Left(NumberNode::new(Token {
                line,
                token_type: TokenType::Number,
                slice: "",
                span,
                column,
            }))),
            op_token: Token {
                line,
                token_type: TokenType::Null,
                slice: "",
                span,
                column,
            },
            right_node: Either::Left(NumberNode::new(Token {
                line,
                token_type: TokenType::Number,
                slice: "",
                span,
                column,
            })),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct UnOpNode<'a> {
    num_node: Either<NumberNode<'a>, IdentifierNode<'a>>,
    op_token: Token<'a>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(num_node: Either<NumberNode<'a>, IdentifierNode<'a>>, op_token: Token<'a>) -> Self {
        Self { num_node, op_token }
    }
}
