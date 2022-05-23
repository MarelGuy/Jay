use either::Either;

/*
Jay parser
Copyright (C) 2022  Loris Cuntreri
*/
use crate::lexer::token::{Span, Token, TokenType};
use crate::parser::ast::declarations::AssignType;
use crate::parser::ast::functions::FunctionNode;
use crate::parser::ast::general::{BlockNode, Nodes};
use crate::parser::ast::loops::WhileNode;
use crate::parser::ast::types::TypeNode;

use super::ast::declarations::{ConstDeclNode, VarDeclNode, VarType};
use super::ast::general::{ConditionNode, Node, ParamNode};
use super::ast::if_else::IfNode;
use super::ast::loops::{ForNode, LoopNode};
use super::ast::math_ops::{BinOpNode, UnOpNode};
use super::ast::types::{CharNode, NumberNode, StringNode};

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
    pub ast: Box<Node<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            ast: Box::new(Node::new(vec![], Box::new(Nodes::NullNode))),
        }
    }

    pub fn parse(&mut self) {
        let mut children: Vec<Box<Node>> = Vec::new();

        while self.tok_i < self.token_stream.len() {
            self.next();

            let node = self.parse_list(self.current_token);

            if node.node != Box::new(Nodes::NullNode) {
                children.push(node);
            }
        }

        self.ast = Box::new(Node::new(children, Box::new(Nodes::NullNode)));
    }

    // Utils
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
            TokenType::String => self.parse_string(),
            TokenType::Char => self.parse_char(),
            TokenType::Let => self.parse_var(false, false),
            TokenType::Var => self.parse_var(true, false),
            TokenType::Const => self.parse_var(false, true),
            TokenType::Type => self.parse_type(),
            TokenType::If => self.parse_if_else(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Loop => self.parse_loop(),
            TokenType::Func => self.parse_function(),
            TokenType::LambFunc => self.parse_lambda(),
            _ => Box::new(Node::new(vec![], Box::new(Nodes::NullNode))),
        }
    }

    // Types
    fn parse_number(&self) -> Box<Node<'a>> {
        let token: Token = self.current_token.clone();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::NumberNode(NumberNode::new(token))),
        ));
    }

    fn parse_string(&self) -> Box<Node<'a>> {
        let token: Token = self.current_token.clone();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::StringNode(StringNode::new(token, token.slice.len()))),
        ));
    }

    fn parse_char(&self) -> Box<Node<'a>> {
        let token: Token = self.current_token.clone();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::CharNode(CharNode::new(token))),
        ));
    }

    fn parse_ty(&mut self) -> VarType {
        match self.current_token.token_type {
            TokenType::IntType => VarType::Int,
            TokenType::FloatType => VarType::Float,
            TokenType::BoolType => VarType::Bool,
            TokenType::StringType => VarType::String,
            TokenType::CharType => VarType::Char,
            TokenType::VoidType => VarType::Void,
            TokenType::Type => VarType::Type,
            _ => VarType::Error,
        }
    }

    // Ops
    fn parse_bin_op(&mut self) -> Box<Node<'a>> {
        let left_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
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
        let number_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;

        self.next();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::UnOpNode(UnOpNode::new(op_token, number_node))),
        ))
    }

    // Declarations
    fn parse_var(&mut self, is_mut: bool, is_const: bool) -> Box<Node<'a>> {
        self.next();
        let mut name: String = self.current_token.slice.into();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();
        self.next();

        let ty: VarType = self.parse_ty();

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

        self.next();

        let mut value: Vec<Box<Node>> = vec![];

        while self.current_token.token_type != TokenType::Semicolon {
            value.push(self.parse_list(self.current_token));
            self.next();
        }

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

    fn parse_type(&mut self) -> Box<Node<'a>> {
        self.next();
        let name: String = self.current_token.slice.into();

        let mut fields: Vec<Box<Node>> = vec![];

        self.next();
        self.next();

        while self.current_token.token_type != TokenType::CloseBrace {
            fields.push(self.parse_param());
            self.next();
        }

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::TypeNode(TypeNode::new(name, fields))),
        ))
    }

    // Statements
    fn parse_condition(&mut self) -> Box<Node<'a>> {
        self.next();

        let left_node = self.current_token.clone();
        self.next();

        let op_token = self.current_token.clone();
        self.next();

        let right_node = self.current_token.clone();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::ConditionNode(ConditionNode::new(
                left_node, op_token, right_node,
            ))),
        ))
    }

    fn parse_block(&mut self) -> Box<Node<'a>> {
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
        let condition: Box<Node> = self.parse_condition();

        self.next();

        let if_block: Box<Node> = self.parse_block();

        self.next();

        if self.current_token.token_type == TokenType::If {}

        if self.current_token.token_type == TokenType::Else {
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

    fn parse_while(&mut self) -> Box<Node<'a>> {
        let condition: Box<Node> = self.parse_condition();

        self.next();

        let while_block: Box<Node> = self.parse_block();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::WhileNode(WhileNode::new(condition, while_block))),
        ))
    }

    fn parse_for(&mut self) -> Box<Node<'a>> {
        let condition: Box<Node> = self.parse_condition();
        self.next();

        let mut next_block: Either<Box<Node<'a>>, ()> = Either::Right(());

        if self.current_token.token_type == TokenType::Next {
            self.next();

            next_block = Either::Left(self.parse_un_op());
        }

        let for_block: Box<Node> = self.parse_block();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::ForNode(ForNode::new(
                condition, next_block, for_block,
            ))),
        ))
    }

    fn parse_loop(&mut self) -> Box<Node<'a>> {
        let loop_block: Box<Node> = self.parse_block();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::LoopNode(LoopNode::new(loop_block))),
        ))
    }

    // Functions
    fn parse_param(&mut self) -> Box<Node<'a>> {
        if self.current_token.token_type == TokenType::Comma {
            self.next();
        }

        let mut name: String = self.current_token.slice.into();

        self.next();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();

        let ty: VarType = self.parse_ty();

        self.next();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::ParamNode(ParamNode::new(name, ty))),
        ))
    }

    fn parse_function(&mut self) -> Box<Node<'a>> {
        self.next();

        let mut name: String = self.current_token.slice.into();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();

        let mut params: Vec<Box<Node>> = vec![];

        self.next();

        while self.current_token.token_type != TokenType::CloseParen {
            params.push(self.parse_param());
        }

        self.next();
        self.next();

        let ret_ty = self.parse_ty();

        self.next();

        let function_block: Box<Node> = self.parse_block();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::FunctionNode(FunctionNode::new(
                name,
                params,
                ret_ty,
                function_block,
            ))),
        ))
    }

    fn parse_lambda(&mut self) -> Box<Node<'a>> {
        self.next();

        let mut params: Vec<Box<Node>> = vec![];

        self.next();

        while self.current_token.token_type != TokenType::CloseParen {
            params.push(self.parse_param());
        }

        self.next();

        self.next();

        let ret_ty: VarType = self.parse_ty();

        self.next();
        self.next();
        self.next();

        let mut block_node: Box<Node> = Box::new(Node::new(vec![], Box::new(Nodes::NullNode)));

        while self.current_token.token_type != TokenType::Semicolon {
            let node: Box<Node> = self.parse_list(self.current_token);
            self.next();

            if node.node != Box::new(Nodes::NullNode) {
                block_node.children.push(node);
            }
        }

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::FunctionNode(FunctionNode::new(
                "".to_string(),
                params,
                ret_ty,
                block_node,
            ))),
        ))
    }
}
