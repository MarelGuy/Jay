use either::Either;
use Either::{Left, Right};
/*
    Jay parser
    Copyright (C) 2022  Loris Cuntreri
*/

use super::ast::declarations::{ConstDeclNode, TypeName, VarDeclNode, VarType};
use super::ast::functions::{ArgNode, FunctionDeclNode, UseFunctionNode};
use super::ast::general::{ConditionNode, Node, ParamNode};
use super::ast::identifier::IdentifierNode;
use super::ast::if_else::IfNode;
use super::ast::loops::{ForNode, LoopNode};
use super::ast::math_ops::{BinOpNode, UnOpNode};
use super::ast::switch::{CaseNode, DefaultNode, SwitchNode};
use super::ast::types::{BoolNode, CharNode, NumberNode, StringNode};

use crate::lexer::token::{Span, Token, TokenType};
use crate::parser::ast::declarations::AssignType;
use crate::parser::ast::functions::FunctionNode;
use crate::parser::ast::general::{BlockNode, Nodes};
use crate::parser::ast::loops::WhileNode;
use crate::parser::ast::types::TypeNode;

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
    pub types: Vec<String>,
    pub functions: Vec<String>,
    pub variables: Vec<String>,
    pub ast: Vec<Box<Node<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            types: Vec::new(),
            functions: Vec::new(),
            variables: Vec::new(),
            ast: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Box<Node<'a>> = self.parse_list(self.current_token);

            if new_node != Box::new(Node::new(Box::new(Nodes::NullNode))) {
                self.ast.push(new_node);
            }
        }
    }

    // Utils
    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    fn back(&mut self) {
        self.tok_i -= 1;

        self.current_token = self.token_stream[self.tok_i];
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

    fn parse_list(&mut self, current_token: Token<'a>) -> Box<Node<'a>> {
        match current_token.token_type {
            TokenType::Number | TokenType::Float => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Multiply
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Power
                    || self.peek().token_type == TokenType::Modulo
                {
                    Box::new(Node::new(Box::new(Nodes::BinOpNode(*self.parse_bin_op()))))
                } else if self.peek().token_type == TokenType::PlusPlus
                    || self.peek().token_type == TokenType::MinusMinus
                {
                    Box::new(Node::new(Box::new(Nodes::UnOpNode(*self.parse_un_op()))))
                } else {
                    Box::new(Node::new(Box::new(Nodes::NumberNode(*self.parse_number()))))
                }
            }
            TokenType::String => {
                Box::new(Node::new(Box::new(Nodes::StringNode(*self.parse_string()))))
            }
            TokenType::Char => Box::new(Node::new(Box::new(Nodes::CharNode(*self.parse_char())))),
            TokenType::BoolType => {
                Box::new(Node::new(Box::new(Nodes::BoolNode(*self.parse_bool()))))
            }
            TokenType::Let => Box::new(Node::new(Box::new(Nodes::VarDeclNode(
                *self.parse_var(false, false).left().unwrap(),
            )))),
            TokenType::Var => Box::new(Node::new(Box::new(Nodes::VarDeclNode(
                *self.parse_var(true, false).left().unwrap(),
            )))),
            TokenType::Const => Box::new(Node::new(Box::new(Nodes::ConstDeclNode(
                *self.parse_var(false, true).right().unwrap(),
            )))),
            TokenType::Type => Box::new(Node::new(Box::new(Nodes::TypeNode(*self.parse_type())))),
            TokenType::If => Box::new(Node::new(Box::new(Nodes::IfNode(*self.parse_if_else())))),
            TokenType::While => {
                Box::new(Node::new(Box::new(Nodes::WhileNode(*self.parse_while()))))
            }
            TokenType::For => Box::new(Node::new(Box::new(Nodes::ForNode(*self.parse_for())))),
            TokenType::Loop => Box::new(Node::new(Box::new(Nodes::LoopNode(*self.parse_loop())))),
            TokenType::Func => Box::new(Node::new(Box::new(Nodes::FunctionNode(
                *self.parse_function(false),
            )))),
            TokenType::Switch => {
                Box::new(Node::new(Box::new(Nodes::SwitchNode(*self.parse_switch()))))
            }
            TokenType::Identifier => match self.peek().token_type {
                TokenType::TripleColon => Box::new(Node::new(Box::new(Nodes::FunctionNode(
                    *self.parse_function(true),
                )))),
                TokenType::DoubleColon => Box::new(Node::new(Box::new(Nodes::UseFunctionNode(
                    *self.parse_use_function(true),
                )))),
                TokenType::OpenParen => Box::new(Node::new(Box::new(Nodes::UseFunctionNode(
                    *self.parse_use_function(false),
                )))),
                _ => Box::new(Node::new(Box::new(Nodes::IdentifierNode(
                    *self.parse_identifier(),
                )))),
            },
            _ => Box::new(Node::new(Box::new(Nodes::NullNode))),
        }
    }

    // Types
    fn parse_number(&self) -> Box<NumberNode<'a>> {
        let token: Token<'a> = self.current_token.clone();

        return Box::new(NumberNode::new(token));
    }

    fn parse_string(&self) -> Box<StringNode<'a>> {
        let token: Token<'a> = self.current_token.clone();

        return Box::new(StringNode::new(token, token.slice.len()));
    }

    fn parse_char(&self) -> Box<CharNode<'a>> {
        let token: Token<'a> = self.current_token.clone();

        return Box::new(CharNode::new(token));
    }

    fn parse_bool(&self) -> Box<BoolNode<'a>> {
        let token: Token<'a> = self.current_token.clone();

        return Box::new(BoolNode::new(token));
    }

    fn parse_ty(&mut self) -> Either<VarType, TypeName> {
        match self.current_token.token_type {
            TokenType::IntType => Left(VarType::Int),
            TokenType::FloatType => Left(VarType::Float),
            TokenType::BoolType => Left(VarType::Bool),
            TokenType::StringType => Left(VarType::String),
            TokenType::CharType => Left(VarType::Char),
            TokenType::VoidType => Left(VarType::Void),
            _ => {
                if self.types.contains(&self.current_token.slice.to_string()) {
                    Right(TypeName::new(self.current_token.slice.to_string()))
                } else {
                    Left(VarType::Error)
                }
            }
        }
    }

    // Identifiers
    fn parse_identifier(&self) -> Box<IdentifierNode<'a>> {
        let token: Token<'a> = self.current_token.clone();

        return Box::new(IdentifierNode::new(token));
    }

    // Ops
    fn parse_bin_op(&mut self) -> Box<BinOpNode<'a>> {
        let left_node: Box<NumberNode<'a>> = self.parse_number();
        self.next();

        let op_token: Token<'a> = self.current_token;
        self.next();

        let right_node: Box<NumberNode<'a>> = self.parse_number();
        self.next();

        return Box::new(BinOpNode::new(left_node, op_token, right_node));
    }

    fn parse_un_op(&mut self) -> Box<UnOpNode<'a>> {
        let number_node: Box<NumberNode<'a>> = self.parse_number();
        self.next();

        let op_token: Token<'a> = self.current_token;
        self.next();

        Box::new(UnOpNode::new(op_token, number_node))
    }

    // Declarations
    fn parse_var(
        &mut self,
        is_mut: bool,
        is_const: bool,
    ) -> Either<Box<VarDeclNode<'a>>, Box<ConstDeclNode<'a>>> {
        self.next();

        let mut name: String = self.current_token.slice.into();
        self.variables.push(name.clone());

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();
        self.next();

        let ty: Either<VarType, TypeName> = self.parse_ty();
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
        let mut value: Vec<Box<Node<'a>>> = vec![];

        for type_name in self.types.clone() {
            if &type_name == self.current_token.slice {
                self.back();
            }
        }

        if ty.is_left() {
            value.append(&mut self.parse_value(false, &ty));
        } else {
            value.append(&mut self.parse_value(true, &ty));
        }

        if is_const {
            return Right(Box::new(ConstDeclNode::new(name, ty, assign_token, value)));
        } else {
            return Left(Box::new(VarDeclNode::new(
                name,
                ty,
                assign_token,
                is_mut,
                value,
            )));
        }
    }

    fn parse_type(&mut self) -> Box<TypeNode> {
        self.next();

        let name: String = self.current_token.slice.into();

        self.types.push(name.clone());

        let mut fields: Vec<Box<ParamNode>> = vec![];

        self.next();
        self.next();

        while self.current_token.token_type != TokenType::CloseBrace {
            if self.current_token.token_type == TokenType::Func {
                fields.push(self.parse_param(true));
            } else {
                fields.push(self.parse_param(false));
            }
            self.next();
        }

        Box::new(TypeNode::new(name, fields))
    }

    fn parse_value(
        &mut self,
        is_type_block: bool,
        ty: &Either<VarType, TypeName>,
    ) -> Vec<Box<Node<'a>>> {
        let mut value: Vec<Box<Node<'a>>> = vec![];

        if ty.is_right() {
            if is_type_block == true {
                while self.current_token.token_type != TokenType::Comma
                    && self.current_token.token_type != TokenType::Semicolon
                {
                    self.next();
                    value.push(self.parse_list(self.current_token));
                }
            }
        } else {
            while self.current_token.token_type != TokenType::Semicolon {
                value.push(self.parse_list(self.current_token));
                self.next();
            }
        }

        value
    }

    // Statements
    fn parse_condition(&mut self) -> Box<ConditionNode<'a>> {
        self.next();

        let left_node: Token<'a> = self.current_token.clone();
        self.next();

        let op_token: Token<'a> = self.current_token.clone();
        self.next();

        let right_node: Token<'a> = self.current_token.clone();

        Box::new(ConditionNode::new(left_node, op_token, right_node))
    }

    fn parse_block(&mut self) -> Box<BlockNode<'a>> {
        self.next();

        let mut block_node: Vec<Box<Node<'a>>> = vec![];

        while self.current_token.token_type != TokenType::CloseBrace {
            let new_node: Box<Node<'a>> = self.parse_list(self.current_token);

            if new_node != Box::new(Node::new(Box::new(Nodes::NullNode))) {
                block_node.push(new_node);
            }

            self.next();
        }

        Box::new(BlockNode::new(block_node))
    }

    fn parse_if_else(&mut self) -> Box<IfNode<'a>> {
        let condition: Box<ConditionNode<'a>> = self.parse_condition();
        self.next();

        let if_block: Box<BlockNode<'a>> = self.parse_block();
        self.next();

        if self.current_token.token_type == TokenType::Else {
            self.next();

            let else_block: Box<BlockNode<'a>> = self.parse_block();

            return Box::new(IfNode::new(condition, if_block, either::Left(else_block)));
        }

        Box::new(IfNode::new(condition, if_block, either::Right(())))
    }

    fn parse_while(&mut self) -> Box<WhileNode<'a>> {
        let condition: Box<ConditionNode<'a>> = self.parse_condition();
        self.next();

        let while_block: Box<BlockNode<'a>> = self.parse_block();

        Box::new(WhileNode::new(condition, while_block))
    }

    fn parse_for(&mut self) -> Box<ForNode<'a>> {
        let condition: Box<ConditionNode<'a>> = self.parse_condition();
        self.next();

        let mut next_block: Either<Box<UnOpNode<'a>>, ()> = Either::Right(());

        if self.current_token.token_type == TokenType::Next {
            self.next();
            next_block = Either::Left(self.parse_un_op());
        }

        let for_block: Box<BlockNode<'a>> = self.parse_block();

        Box::new(ForNode::new(condition, next_block, for_block))
    }

    fn parse_loop(&mut self) -> Box<LoopNode<'a>> {
        let loop_block: Box<BlockNode<'a>> = self.parse_block();
        Box::new(LoopNode::new(loop_block))
    }

    // Switch
    fn parse_switch(&mut self) -> Box<SwitchNode<'a>> {
        self.next();

        let mut cases: Vec<Box<CaseNode<'a>>> = vec![];

        let mut default_node: Box<DefaultNode<'a>> =
            Box::new(DefaultNode::new(Box::new(BlockNode::new(vec![]))));

        let mut is_default: bool = false;
        self.next();

        while self.current_token.token_type != TokenType::CloseBrace {
            if self.current_token.token_type == TokenType::Default {
                is_default = true;
                default_node = self.parse_default();
            } else {
                cases.push(self.parse_case());
            }

            self.next();
        }

        if is_default == true {
            Box::new(SwitchNode::new(cases, Left(default_node)))
        } else {
            Box::new(SwitchNode::new(cases, Right(())))
        }
    }

    fn parse_case(&mut self) -> Box<CaseNode<'a>> {
        let condition: Box<ConditionNode<'a>> = self.parse_condition();
        self.next();

        let case_block: Box<BlockNode<'a>> = self.parse_block();

        Box::new(CaseNode::new(condition, case_block))
    }

    fn parse_default(&mut self) -> Box<DefaultNode<'a>> {
        let case_block: Box<BlockNode<'a>> = self.parse_block();

        Box::new(DefaultNode::new(case_block))
    }

    // Functions
    fn parse_function_decl(&mut self, is_from_type: bool) -> Box<FunctionDeclNode> {
        self.next();

        if is_from_type == true {
            self.next();
        }

        let mut name: String = self.current_token.slice.into();

        if is_from_type == false {
            self.functions.push(name.clone());
        }

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();
        self.next();

        let mut params: Vec<Box<ParamNode>> = vec![];

        while self.current_token.token_type != TokenType::CloseParen {
            if self.current_token.token_type == TokenType::Func {
                params.push(self.parse_param(true));
            } else {
                params.push(self.parse_param(false));
            }
        }

        self.next();
        self.next();

        let ret_ty: Either<VarType, TypeName> = self.parse_ty();

        Box::new(FunctionDeclNode::new(name, params, ret_ty))
    }

    fn parse_function(&mut self, is_from_type: bool) -> Box<FunctionNode<'a>> {
        let func_details: Box<FunctionDeclNode> = self.parse_function_decl(is_from_type);

        self.next();

        let function_block: Box<BlockNode<'a>> = self.parse_block();

        Box::new(FunctionNode::new(func_details, function_block))
    }

    fn parse_use_function(&mut self, is_from_type: bool) -> Box<UseFunctionNode<'a>> {
        if is_from_type == true {
            self.next();
            self.next();
        }

        let name: String = self.current_token.slice.into();

        self.next();
        self.next();

        let mut args: Vec<Box<ArgNode>> = vec![];

        while self.current_token.token_type != TokenType::CloseParen {
            if self.current_token.token_type == TokenType::Comma {
                self.next();
            }

            args.push(self.parse_arg());

            self.next();
        }

        self.next();
        self.next();

        Box::new(UseFunctionNode::new(name, args))
    }

    // Params
    fn parse_param(&mut self, is_func: bool) -> Box<ParamNode> {
        if self.current_token.token_type == TokenType::Comma {
            self.next();
        }

        if is_func == true {
            let func_details: Box<FunctionDeclNode> = self.parse_function_decl(false);

            self.next();

            return Box::new(ParamNode::new(
                func_details.name.clone(),
                Left(*func_details),
            ));
        }

        let mut name: String = self.current_token.slice.into();
        self.next();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        let ty: Either<FunctionDeclNode, Either<VarType, TypeName>>;
        self.next();

        ty = Either::Right(self.parse_ty());
        self.next();

        Box::new(ParamNode::new(name, ty))
    }

    fn parse_arg(&mut self) -> Box<ArgNode<'a>> {
        let value: Box<Node<'a>> = self.parse_list(self.current_token);

        Box::new(ArgNode::new(value))
    }
}
