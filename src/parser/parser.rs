/*
Jay parser
Copyright (C) 2022  Loris Cuntreri
*/
use crate::lexer::token::{Span, Token, TokenType};
use crate::parser::ast::declarations::AssignType;
use crate::parser::ast::general::{BlockNode, Nodes};

use super::ast::declarations::{ConstDeclNode, VarDeclNode, VarType};
use super::ast::general::{ConditionNode, Node};
use super::ast::if_else::IfNode;
use super::ast::math_ops::{BinOpNode, UnOpNode};
use super::ast::types::NumberNode;

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

    fn parse_list(&mut self, current_token: Token) -> Box<Node<'a>> {
        match current_token.token_type {
            TokenType::Number | TokenType::Float => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Multiply
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Power
                    || self.peek().token_type == TokenType::Modulo
                {
                    self.parse_bin_op()
                } else if self.peek().token_type == TokenType::PlusPlus
                    || self.peek().token_type == TokenType::MinusMinus
                {
                    self.parse_un_op()
                } else {
                    self.parse_number()
                }
            }
            TokenType::Let => {
                println!("LET_DECL");
                self.parse_var(false, false)
            }
            TokenType::Var => {
                println!("VAR_DECL");
                self.parse_var(true, false)
            }
            TokenType::Const => {
                println!("CONST_DECL");
                self.parse_var(false, true)
            }
            TokenType::If => self.parse_if_else(),
            _ => Box::new(Node::new(vec![], Box::new(Nodes::NullNode))),
        }
    }

    pub fn parse(&mut self) -> Box<Node<'a>> {
        let mut children: Vec<Box<Node>> = Vec::new();

        while self.tok_i < self.token_stream.len() {
            self.next();

            let node = self.parse_list(self.current_token);

            if node.node != Box::new(Nodes::NullNode) {
                children.push(node);
            }
        }

        Box::new(Node::new(children, Box::new(Nodes::NullNode)))
    }

    fn parse_number(&self) -> Box<Node<'a>> {
        let token: Token = self.current_token.clone();
        println!("num: {:?}", self.current_token.slice);

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::NumberNode(NumberNode::new(token))),
        ));
    }

    fn parse_bin_op(&mut self) -> Box<Node<'a>> {
        println!("BIN_OP");
        let left_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("binop_type: {:?}", self.current_token.token_type);
        self.next();

        let right_node: Box<Node> = self.parse_number();

        self.next();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::BinOpNode(BinOpNode::new(
                left_node, op_token, right_node,
            ))),
        ));
    }

    fn parse_un_op(&mut self) -> Box<Node<'a>> {
        println!("UN_OP");
        let number_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("unop_type: {:?}", self.current_token.token_type);

        self.next();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::UnOpNode(UnOpNode::new(op_token, number_node))),
        ))
    }

    fn parse_var(&mut self, is_mut: bool, is_const: bool) -> Box<Node<'a>> {
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

        self.next();

        if is_const {
            Box::new(Node::new(
                vec![],
                Box::new(Nodes::ConstDeclNode(ConstDeclNode::new(
                    name,
                    ty,
                    assign_token,
                    value,
                ))),
            ))
        } else {
            Box::new(Node::new(
                vec![],
                Box::new(Nodes::VarDeclNode(VarDeclNode::new(
                    name,
                    ty,
                    assign_token,
                    is_mut,
                    value,
                ))),
            ))
        }
    }

    fn parse_condition(&mut self) -> Box<Node<'a>> {
        println!("CONDITION");
        self.next();
        let left_node = self.current_token.clone();
        println!("left: {:?}", self.current_token.slice);

        self.next();
        let op_token = self.current_token.clone();
        println!("operator: {:?}", self.current_token.token_type);

        self.next();
        let right_node = self.current_token.clone();
        println!("right: {:?}", self.current_token.slice);

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::ConditionNode(ConditionNode::new(
                left_node, op_token, right_node,
            ))),
        ))
    }

    fn parse_block(&mut self) -> Box<Node<'a>> {
        println!("BLOCK");

        self.next();

        let mut block_node: Box<Node> = Box::new(Node::new(vec![], Box::new(Nodes::NullNode)));

        while self.current_token.token_type != TokenType::CloseBrace {
            let node: Box<Node> = self.parse_list(self.current_token);
            self.next();

            if node.node != Box::new(Nodes::NullNode) {
                block_node.children.push(node);
            }
        }

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::BlockNode(BlockNode::new(block_node))),
        ))
    }

    fn parse_if_else(&mut self) -> Box<Node<'a>> {
        println!("IF");
        let condition: Box<Node> = self.parse_condition();
        self.next();

        let if_block: Box<Node> = self.parse_block();
        self.next();

        if self.current_token.token_type == TokenType::If {}

        if self.current_token.token_type == TokenType::Else {
            println!("ELSE");

            self.next();

            let else_block: Box<Node> = self.parse_block();

            return Box::new(Node::new(
                vec![],
                Box::new(Nodes::IfNode(IfNode::new(
                    condition,
                    if_block,
                    either::Left(else_block),
                ))),
            ));
        }

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::IfNode(IfNode::new(
                condition,
                if_block,
                either::Right(()),
            ))),
        ))
    }
}
