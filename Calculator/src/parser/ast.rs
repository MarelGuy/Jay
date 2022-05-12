use crate::lexer::token::Token;

#[derive(PartialEq, Debug)]
pub enum Nodes<'a> {
    BinOpNode(BinOpNode<'a>),
    UnOpNode(UnOpNode<'a>),
    NumberNode(NumberNode<'a>),
    NullNode,
}

#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    pub children: Vec<Box<Node<'a>>>,
    pub node: Box<Nodes<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(children: Vec<Box<Node<'a>>>, node: Box<Nodes<'a>>) -> Self {
        Self { children, node }
    }
}

#[derive(PartialEq, Debug)]
pub struct NumberNode<'a> {
    token: Token<'a>,
}

impl<'a> NumberNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(PartialEq, Debug)]
pub struct BinOpNode<'a> {
    left_node: Box<Node<'a>>,
    op_token: Token<'a>,
    right_node: Box<Node<'a>>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(left_node: Box<Node<'a>>, op_token: Token<'a>, right_node: Box<Node<'a>>) -> Self {
        Self {
            left_node,
            op_token,
            right_node,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct UnOpNode<'a> {
    op_token: Token<'a>,
    node: Box<Node<'a>>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(op_token: Token<'a>, node: Box<Node<'a>>) -> Self {
        Self { op_token, node }
    }
}
