use either::Either::{self, Left, Right};

use crate::{error_handler::Error, lexer::token::Span};
use std::{process::exit, vec};

use crate::lexer::token::{Token, TokenType};

use self::ast::{
    primitive_node::PrimitiveTypeNode,
    variables::{ArrElem, ArrayVarType, CallVarArrNode, CallVarNode, VarNode, VarType},
    Node, Nodes,
};

pub(crate) mod ast;
mod math;

pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    file_name: String,
    lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Node<'a>>,

    // Vectors
    var_vec: Vec<VarNode<'a>>,
}

impl<'a> Parser<'a> {
    // Main functions
    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        Self {
            file_name,
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],
            var_vec: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Node<'a> = self.parse_list(self.current_token);

            if new_node != Node(Nodes::NullNode) {
                self.ast.push(new_node);
            }
        }
    }

    // Flow functions
    fn back(&mut self) {
        self.tok_i -= 1;

        self.current_token = self.token_stream[self.tok_i];
    }

    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    // Utils functions
    fn peek(&self) -> Token<'a> {
        if self.tok_i < self.token_stream.len() {
            self.token_stream[self.tok_i].clone()
        } else {
            Token {
                token_type: TokenType::Null,
                slice: "",
                span: Span { start: 0, end: 0 },
                line: 0,
                column: 0,
            }
        }
    }

    fn get_ty(&self) -> VarType {
        match self.current_token.token_type {
            TokenType::IntType => VarType::Int,
            TokenType::FloatType => VarType::Float,
            TokenType::StringType => VarType::String,
            TokenType::BoolType => VarType::Bool,
            TokenType::CharType => VarType::Char,
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_ty_not_found();
                return VarType::Type {
                    name: "Null".to_string(),
                };
            }
        }
    }

    fn get_ty_from_val(&self) -> VarType {
        match self.current_token.token_type {
            TokenType::Number => VarType::Int,
            TokenType::Float => VarType::Float,
            TokenType::String => VarType::String,
            TokenType::Bool => VarType::Bool,
            TokenType::Char => VarType::Char,
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_ty_not_found();
                return VarType::Type {
                    name: "Null".to_string(),
                };
            }
        }
    }

    fn get_array_ty(&mut self) -> ArrayVarType {
        let type_token: Token = self.current_token;

        self.next();
        self.next();

        let init_num: isize = self.current_token.slice.parse::<isize>().unwrap();

        match type_token.token_type {
            TokenType::IntType => ArrayVarType::Int { init_num },
            TokenType::FloatType => ArrayVarType::Float { init_num },
            TokenType::StringType => ArrayVarType::String { init_num },
            TokenType::BoolType => ArrayVarType::Bool { init_num },
            TokenType::CharType => ArrayVarType::Char { init_num },
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_ty_not_found();
                return ArrayVarType::Type {
                    name: "Null".to_string(),
                    init_num,
                };
            }
        }
    }

    fn search_var_arr(&mut self, string_to_search: String) -> Result<usize, usize> {
        self.var_vec
            .clone()
            .into_iter()
            .map(|x| -> String { x.name })
            .collect::<Vec<String>>()
            .binary_search(&string_to_search)
    }

    #[rustfmt::skip]    fn get_line(&self, line: usize) -> String { self.lines.clone().into_iter().nth(line).unwrap() }

    // Parser
    fn parse_list(&mut self, token: Token<'a>) -> Node<'a> {
        match token.token_type {
            TokenType::Semicolon => Node(Nodes::EOL),
            TokenType::Number
            | TokenType::Float
            | TokenType::String
            | TokenType::Char
            | TokenType::Bool
            | TokenType::Identifier
            | TokenType::NegativeFloat
            | TokenType::NegativeNumber => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Multiply
                {
                    let mut tok_stream: Vec<Token> = vec![];

                    loop {
                        tok_stream.push(self.current_token);

                        self.next();

                        if self.current_token.token_type == TokenType::Semicolon
                            || self.current_token.token_type == TokenType::Colon
                            || self.current_token.token_type == TokenType::CloseBracket
                        {
                            self.back();
                            break;
                        }
                    }

                    Node(Nodes::ProcessedMathNode(math::process_math_node(
                        tok_stream,
                        self.get_line(self.current_token.line),
                        self.file_name.clone(),
                    )))
                } else {
                    return self
                        .search_var_arr(self.current_token.slice.into())
                        .is_ok()
                        .then(|| {
                            if self.peek().token_type == TokenType::OpenBracket {
                                Node(Nodes::CallVarArrNode(self.parse_call_var_arr_node()))
                            } else {
                                Node(Nodes::CallVarNode(self.parse_call_var_node()))
                            }
                        })
                        .unwrap_or_else(|| {
                            if self.current_token.token_type == TokenType::Identifier {
                                Error::new(
                                    self.current_token,
                                    self.get_line(self.current_token.line),
                                    self.file_name.clone(),
                                )
                                .throw_var_not_defined()
                            }

                            Node(Nodes::PrimitiveTypeNode(self.parse_primitive_type_node()))
                        });
                }
            }
            TokenType::Let | TokenType::Var | TokenType::Const => {
                Node(Nodes::VarNode(self.parse_var()))
            }
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_unkown_token();
                Node(Nodes::NullNode)
            }
        }
    }

    #[rustfmt::skip]    fn parse_primitive_type_node(&mut self) -> PrimitiveTypeNode<'a> { PrimitiveTypeNode(self.current_token) }

    fn parse_var(&mut self) -> VarNode<'a> {
        let mut is_mut: bool = false;

        if self.current_token.token_type == TokenType::Var {
            is_mut = true;
        }

        self.next();

        let name: String = self.current_token.slice.into();

        if name.chars().nth(0).unwrap().is_numeric() {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_cant_start_var_num();
        };

        self.search_var_arr(name.clone()).is_ok().then(|| {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_cant_use_same_var_name();
        });

        self.next();

        self.next();

        let ty: Either<VarType, ArrayVarType>;

        if self.peek().token_type == TokenType::OpenBracket {
            ty = Right(self.get_array_ty());

            self.next();
        } else {
            ty = Left(self.get_ty());
        }

        self.next();
        self.next();

        loop {
            if self.current_token.token_type == TokenType::OpenBracket {
                let mut index: isize = 0;
                let mut value: Vec<ArrElem<'a>> = vec![];

                self.next();

                loop {
                    if self.get_ty_from_val() != ty.clone().right().unwrap().to_var_type() {
                        Error::new(
                            self.current_token,
                            self.get_line(self.current_token.line),
                            self.file_name.clone(),
                        )
                        .throw_wrong_assign_type(
                            &name,
                            self.get_ty_from_val().to_string(),
                            ty.clone().right().unwrap().to_var_type().to_string(),
                        );
                    }

                    if &index == ty.clone().right().unwrap().get_init_num() {
                        Error::new(
                            self.current_token,
                            self.get_line(self.current_token.line),
                            self.file_name.clone(),
                        )
                        .throw_array_out_of_bounds(ty.clone().right().unwrap().get_init_num());
                    }

                    value.push(ArrElem::new(
                        Box::new(self.parse_list(self.current_token)),
                        index,
                    ));

                    self.next();

                    if self.current_token.token_type == TokenType::Comma {
                        index += 1;
                        self.next();
                    } else if self.current_token.token_type == TokenType::CloseBracket {
                        break;
                    }
                }

                let new_node: VarNode = VarNode::new(name, ty, Either::Right(value), is_mut);

                self.var_vec.push(new_node.clone());

                return new_node;
            }

            if self.get_ty_from_val() != ty.clone().left().unwrap() {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_wrong_assign_type(
                    &name,
                    self.get_ty_from_val().to_string(),
                    ty.clone().left().unwrap().to_string(),
                );
            }
            let value: Node<'a> = self.parse_list(self.current_token);

            self.next();

            if self.current_token.token_type == TokenType::Semicolon {
                self.back();

                let new_node: VarNode =
                    VarNode::new(name, ty, Either::Left(Box::new(value)), is_mut);

                self.var_vec.push(new_node.clone());

                return new_node;
            }
        }
    }

    // TODO: Error Handling for both functions
    fn parse_call_var_node(&mut self) -> CallVarNode<'a> {
        let var_to_call: VarNode = self
            .var_vec
            .clone()
            .into_iter()
            .nth(
                self.search_var_arr(self.current_token.slice.into())
                    .unwrap(),
            )
            .unwrap();

        self.next();

        CallVarNode::new(var_to_call)
    }

    fn parse_call_var_arr_node(&mut self) -> CallVarArrNode<'a> {
        let var_to_call: CallVarNode = self.parse_call_var_node();

        self.next();

        let index: isize = self.current_token.slice.parse().unwrap();

        if index < 0
            || var_to_call
                .var_to_call
                .clone()
                .val
                .right()
                .unwrap()
                .into_iter()
                .last()
                .unwrap()
                .index
                < index
        {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_cant_use_num_array(var_to_call.var_to_call.name.as_str());
        }

        self.next();

        CallVarArrNode::new(var_to_call, index)
    }
}
