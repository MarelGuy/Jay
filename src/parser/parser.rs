/*
Jay parser
Copyright (C) 2022  Loris Cuntreri
*/
use crate::lexer::token::{Span, Token, TokenType};
use crate::parser::ast::declarations::AssignType;

use super::ast::declarations::{ConstDeclNode, VarDeclNode, VarType};
use super::ast::general::{BlockNode, ConditionNode, Node};
use super::ast::if_else::IfNode;
use super::ast::math_ops::{BinOpNode, UnOpNode};
use super::ast::types::NumberNode;

use either::Either;

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
        }
    }

    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    fn peek(&self) -> Token<'a> {
        if self.tok_i < self.token_stream.len() {
            self.token_stream[self.tok_i].clone()
        } else {
            Token {
                token_type: TokenType::Null,
                slice: "",
                span: Span { start: 0, end: 0 },
            }
        }
    }

    pub fn parse(&mut self) {
        for _ in 0..self.token_stream.len() {
            self.next();
            match self.current_token.token_type {
                TokenType::Number | TokenType::Float => {
                    if self.peek().token_type == TokenType::Plus
                        || self.peek().token_type == TokenType::Minus
                        || self.peek().token_type == TokenType::Multiply
                        || self.peek().token_type == TokenType::Divide
                        || self.peek().token_type == TokenType::Power
                        || self.peek().token_type == TokenType::Modulo
                    {
                        self.parse_bin_op();
                    } else if self.peek().token_type == TokenType::PlusPlus
                        || self.peek().token_type == TokenType::MinusMinus
                    {
                        self.parse_un_op();
                    } else {
                        self.parse_number();
                    }
                }
                TokenType::Let => {
                    self.parse_var(false, false);
                    // useless comment, yay!
                }
                TokenType::Var => {
                    self.parse_var(true, false);
                    // another useless comment, even more yay!
                }
                TokenType::Const => {
                    self.parse_var(false, true);
                }
                TokenType::If => {
                    self.parse_if_else();
                }
                _ => self.next(),
            };
        }
    }

    fn parse_number(&self) -> Node<NumberNode<'a>> {
        println!("num: {:?}", self.current_token.slice);
        let token: Token = self.current_token.clone();

        Node::new(vec![], NumberNode::new(token))
    }

    fn parse_bin_op(&mut self) -> Node<BinOpNode<'a>> {
        let left_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("binop_type: {:?}", self.current_token.token_type);
        self.next();

        let right_node: Node<NumberNode> = self.parse_number();

        Node::new(vec![], BinOpNode::new(left_node, op_token, right_node))
    }

    fn parse_un_op(&mut self) -> Node<UnOpNode<'a>> {
        let number_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("unop_type: {:?}", self.current_token.token_type);

        Node::new(vec![], UnOpNode::new(op_token, number_node))
    }

    fn parse_var(
        &mut self,
        is_mut: bool,
        is_const: bool,
    ) -> Either<Node<VarDeclNode>, Node<ConstDeclNode>> {
        self.next();
        let mut name: String = self.current_token.slice.into();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        println!("name: {:?}", name);

        self.next();
        self.next();

        let ty: VarType = match self.current_token.token_type {
            TokenType::IntType => VarType::Int,
            TokenType::FloatType => VarType::Float,
            TokenType::BoolType => VarType::Bool,
            TokenType::StringType => VarType::String,
            TokenType::CharType => VarType::Char,
            _ => VarType::Error,
        };
        println!("type: {:?}", ty);

        self.next();
        let assign_token: AssignType = match self.current_token.token_type {
            TokenType::Assign => AssignType::Assign,
            TokenType::PlusAssign => AssignType::AddAssign,
            TokenType::MinusAssign => AssignType::SubAssign,
            TokenType::MultiplyAssign => AssignType::MulAssign,
            TokenType::DivideAssign => AssignType::DivAssign,
            TokenType::ModuloAssign => AssignType::ModAssign,
            TokenType::PowerAssign => AssignType::PowAssign,
            _ => AssignType::Error,
        };
        println!("assign_type: {:?}", assign_token);

        self.next();

        let value: String = self.current_token.slice.into();
        println!("value: {:?}", value);

        println!("is_mut: {:?}", is_mut);
        println!("is_const: {:?}", is_const);

        if is_const {
            either::Right(Node::new(
                vec![],
                ConstDeclNode::new(name, ty, assign_token, value),
            ))
        } else {
            either::Left(Node::new(
                vec![],
                VarDeclNode::new(name, ty, assign_token, is_mut, value),
            ))
        }
    }

    fn parse_condition(&mut self) -> Node<ConditionNode<'a>> {
        self.next();
        let left_node = self.current_token.clone();
        println!("left: {:?}", self.current_token.slice);

        self.next();
        let op_token = self.current_token.clone();
        println!("operator: {:?}", self.current_token.token_type);

        self.next();
        let right_node = self.current_token.clone();
        println!("right: {:?}", self.current_token.slice);

        Node::new(vec![], ConditionNode::new(left_node, op_token, right_node))
    }

    fn parse_block(&mut self) -> Node<BlockNode> {
        self.next();

        let mut raw_block: Vec<String> = Vec::new();

        while self.current_token.token_type != TokenType::CloseBrace {
            raw_block.push(self.current_token.slice.into());
            self.next();

            if self.current_token.token_type == TokenType::LineFeed {
                raw_block.clear();
                raw_block.push("Error".to_string());
                break;
            }
        }

        let block: String = raw_block.join(" ");
        println!("block: {:?}", block);

        Node::new(vec![], BlockNode::new(block))
    }

    fn parse_if_else(&mut self) -> Node<IfNode> {
        let condition: Node<ConditionNode<'a>> = self.parse_condition();
        self.next();

        let if_block: Node<BlockNode> = self.parse_block();
        self.next();

        if self.current_token.token_type == TokenType::Else {
            self.next();
            let else_block: Node<BlockNode> = self.parse_block();
            return Node::new(
                vec![],
                IfNode::new(condition, if_block, either::Left(else_block)),
            );
        }
        Node::new(vec![], IfNode::new(condition, if_block, either::Right(())))
    }
}
