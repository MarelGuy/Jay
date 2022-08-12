use either::Either;

use super::Node;

#[derive(Debug, PartialEq)]
pub struct ImportNode<'a> {
    pub import: Either<Box<Node<'a>>, Vec<Box<Node<'a>>>>,
    pub from: Box<Node<'a>>,
}

impl<'a> ImportNode<'a> {
    pub fn new(import: Either<Box<Node<'a>>, Vec<Box<Node<'a>>>>, from: Box<Node<'a>>) -> Self {
        Self { import, from }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportNode<'a> {
    pub items: Either<Box<Node<'a>>, Vec<Box<Node<'a>>>>,
}

impl<'a> ExportNode<'a> {
    pub fn new(items: Either<Box<Node<'a>>, Vec<Box<Node<'a>>>>) -> Self {
        Self { items }
    }
}
