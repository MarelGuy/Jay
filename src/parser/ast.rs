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

pub struct BinOpNode<'a, T> {
    left: Box<T>,
    token: Token<'a>,
    right: Box<T>,
}
