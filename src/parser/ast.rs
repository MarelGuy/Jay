/*
 Jay AST
 Copyright (C) 2022 Loris Cuntreri
*/

use crate::lexer::token::Token;

pub struct NumberNode<'a> {
    pub token: Token<'a>,
}

pub struct Node<'a> {
    token: Token<'a>,
    children: Vec<Node<'a>>,
}

pub struct BinOpNode<'a> {
    token: Token<'a>,
    left: Box<Node<'a>>,
    right: Box<Node<'a>>,
}


