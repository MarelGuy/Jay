use either::Either::{self, Left, Right};
use std::vec;

use crate::lexer::token::{Token, TokenType};
use crate::{error_handler::Error, lexer::token::Span};

use self::ast::variables::AssignToVarArrNode;
use self::ast::{
    primitive_node::PrimitiveTypeNode,
    variables::{
        ArrElem, ArrayVarType, AssignToVarNode, CallVarArrNode, CallVarNode, VarNode, VarType,
    },
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

    error_handler: Error<'a>,

    // Vectors
    var_vec: Vec<VarNode<'a>>,
}

impl<'a> Parser<'a> {
    // Main functions
    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        let init_tok: Token = token_stream[0].clone();

        Self {
            file_name: file_name.clone(),
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],

            error_handler: Error::new(init_tok, "".into(), file_name),

            var_vec: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Node<'a> = self.parse_list(self.current_token);

            self.ast.push(new_node);
        }
    }

    fn get_line(&self, line: usize) -> String {
        self.lines.clone().into_iter().nth(line).unwrap()
    }

    fn update_error_handler(&mut self) {
        self.error_handler.token = self.current_token;
        self.error_handler.line_string = self.get_line(self.current_token.line);
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

    // Type functions

    fn get_ty(&mut self) -> VarType {
        match self.current_token.token_type {
            TokenType::IntType => VarType::Int,
            TokenType::FloatType => VarType::Float,
            TokenType::StringType => VarType::String,
            TokenType::BoolType => VarType::Bool,
            TokenType::CharType => VarType::Char,
            _ => {
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

                return VarType::Type {
                    name: "Null".to_string(),
                };
            }
        }
    }

    fn get_ty_from_val(&mut self, token: Token) -> VarType {
        match token.token_type {
            TokenType::Number => VarType::Int,
            TokenType::NegativeNumber => VarType::Int,
            TokenType::Float => VarType::Float,
            TokenType::NegativeFloat => VarType::Float,
            TokenType::String => VarType::String,
            TokenType::Bool => VarType::Bool,
            TokenType::Char => VarType::Char,
            _ => {
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

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
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

                return ArrayVarType::Type {
                    name: "Null".to_string(),
                    init_num,
                };
            }
        }
    }

    // Search vecs functions

    fn search_var_arr(&mut self, string_to_search: String) -> Result<usize, usize> {
        self.var_vec
            .clone()
            .into_iter()
            .map(|x| -> String { x.0 })
            .collect::<Vec<String>>()
            .binary_search(&string_to_search)
    }

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
                            let mut call_var_node: Node;
                            let is_var_node: bool;

                            if self.peek().token_type == TokenType::OpenBracket {
                                call_var_node =
                                    Node(Nodes::CallVarArrNode(self.parse_call_var_arr_node()));
                                is_var_node = false;
                            } else {
                                call_var_node =
                                    Node(Nodes::CallVarNode(self.parse_call_var_node()));
                                is_var_node = true;
                            }

                            match self.peek().token_type {
                                TokenType::Assign => {
                                    if is_var_node == true {
                                        call_var_node = Node(Nodes::AssignToVarNode(
                                            self.parse_assign_to_var(call_var_node),
                                        ));
                                    } else {
                                        call_var_node = Node(Nodes::AssignToVarArrNode(
                                            self.parse_assign_to_var_arr(call_var_node),
                                        ));
                                    }
                                }
                                _ => (),
                            }

                            call_var_node
                        })
                        .unwrap_or_else(|| {
                            if self.current_token.token_type == TokenType::Identifier {
                                self.update_error_handler();
                                self.error_handler.throw_var_not_defined()
                            }

                            Node(Nodes::PrimitiveTypeNode(self.parse_primitive_type_node()))
                        });
                }
            }
            TokenType::Let | TokenType::Var | TokenType::Const => {
                Node(Nodes::VarNode(self.parse_var()))
            }
            _ => {
                self.update_error_handler();
                self.error_handler.throw_unkown_token();
                Node(Nodes::NullNode)
            }
        }
    }

    fn parse_primitive_type_node(&mut self) -> PrimitiveTypeNode<'a> {
        PrimitiveTypeNode(self.current_token)
    }

    // Variables
    fn parse_var(&mut self) -> VarNode<'a> {
        let mut is_mut: bool = false;

        if self.current_token.token_type == TokenType::Var {
            is_mut = true;
        }

        self.next();

        let name: String = self.current_token.slice.into();

        if name.chars().nth(0).unwrap().is_numeric() {
            self.update_error_handler();
            self.error_handler.throw_cant_start_var_num();
        };

        self.search_var_arr(name.clone()).is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_cant_use_same_var_name();
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
                self.next();

                let mut index: isize = 0;
                let mut value: Vec<ArrElem<'a>> = vec![];

                loop {
                    let val_ty: VarType = self.get_ty_from_val(self.current_token);

                    if val_ty != ty.clone().unwrap_right().to_var_type() {
                        self.update_error_handler();
                        self.error_handler.throw_wrong_assign_type(
                            &name,
                            val_ty.to_string(),
                            ty.clone().unwrap_right().to_var_type().to_string(),
                        );
                    }

                    if &index == ty.clone().unwrap_right().get_init_num() {
                        self.update_error_handler();
                        self.error_handler
                            .throw_array_out_of_bounds(ty.clone().unwrap_right().get_init_num());
                    }

                    value.push(ArrElem(
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

                let new_node: VarNode = VarNode(name, ty, Either::Right(value), is_mut);

                self.var_vec.push(new_node.clone());

                return new_node;
            }

            let val_ty: VarType = self.get_ty_from_val(self.current_token);

            if val_ty != ty.clone().unwrap_left() {
                self.update_error_handler();
                self.error_handler.throw_wrong_assign_type(
                    &name,
                    val_ty.to_string(),
                    ty.clone().unwrap_left().to_string(),
                );
            }

            let new_node: VarNode = VarNode(
                name,
                ty,
                Either::Left(Box::new(self.parse_list(self.current_token))),
                is_mut,
            );

            self.var_vec.push(new_node.clone());

            return new_node;
        }
    }

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

        CallVarNode(var_to_call)
    }

    fn parse_index(&mut self) -> isize {
        let index_to_return: isize;

        if self.current_token.token_type == TokenType::Identifier {
            let node_to_parse: Box<Node<'a>>;

            if self
                .search_var_arr(self.current_token.slice.into())
                .is_err()
                == true
            {
                self.update_error_handler();
                self.error_handler.throw_var_not_defined()
            }

            if self.peek().token_type == TokenType::OpenBracket {
                let idk0: CallVarArrNode = self.parse_call_var_arr_node();

                node_to_parse = idk0
                    .0
                     .0
                     .2
                    .right()
                    .unwrap()
                    .into_iter()
                    .nth(idk0.1 as usize)
                    .unwrap()
                    .0;
            } else {
                node_to_parse = self.parse_call_var_node().0 .2.unwrap_left();
            }

            let unpacked_node: Token = node_to_parse.0.get_primitive().unwrap();

            if unpacked_node.token_type != TokenType::Number {
                let val_ty: VarType = self.get_ty_from_val(unpacked_node);

                self.update_error_handler();
                self.error_handler
                    .throw_cant_use_val_in_arr_call(val_ty.to_string());
            }

            index_to_return = unpacked_node.slice.parse().unwrap();
        } else {
            if self.current_token.token_type != TokenType::Number {
                let val_ty: VarType = self.get_ty_from_val(self.current_token);

                self.update_error_handler();
                self.error_handler
                    .throw_cant_use_val_in_arr_call(val_ty.to_string());
            }

            index_to_return = self.current_token.slice.parse().unwrap();
        }

        index_to_return
    }

    fn parse_call_var_arr_node(&mut self) -> CallVarArrNode<'a> {
        let var_to_call: CallVarNode = self.parse_call_var_node();

        self.next();
        self.next();

        let index_to_call: isize = self.parse_index();

        self.next();

        while self.current_token.token_type == TokenType::CloseBracket {
            self.next();
        }

        self.back();

        if index_to_call < 0
            || var_to_call
                .0
                .clone()
                .2
                .right()
                .unwrap()
                .into_iter()
                .last()
                .unwrap()
                .1
                < index_to_call
        {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_cant_use_num_array(var_to_call.0 .0.as_str(), index_to_call);
        }

        CallVarArrNode(var_to_call, index_to_call)
    }

    fn parse_assign_to_var(&mut self, var_to_assign: Node<'a>) -> AssignToVarNode<'a> {
        self.next();
        self.next();

        let var: CallVarNode<'a> = var_to_assign.0.get_call_var_node().unwrap();
        let val: Box<Node<'a>>;

        let var_ty: VarType = var.0 .1.clone().unwrap_left();

        if var_ty != self.get_ty_from_val(self.current_token) {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_wrong_assign_type(
                &var.0 .0,
                self.get_ty_from_val(self.current_token).to_string(),
                var_ty.to_string(),
            );
        }

        val = Box::new(self.parse_list(self.current_token));

        AssignToVarNode(var, val)
    }

    fn parse_assign_to_var_arr(&mut self, var_to_assign: Node<'a>) -> AssignToVarArrNode<'a> {
        self.back();
        self.back();

        let var: CallVarArrNode<'a> = var_to_assign.0.get_call_var_arr_node().unwrap();
        let index: isize = self.current_token.slice.parse().unwrap();

        for _ in 0..4 {
            self.next();
        }

        let var_ty: VarType = var.0 .0 .1.clone().right().unwrap().to_var_type();

        if var_ty != self.get_ty_from_val(self.current_token) {
            Error::new(
                self.current_token,
                self.get_line(self.current_token.line),
                self.file_name.clone(),
            )
            .throw_wrong_assign_type(
                &var.0 .0 .0,
                self.get_ty_from_val(self.current_token).to_string(),
                var_ty.to_string(),
            );
        }

        let val: Box<Node> = Box::new(self.parse_list(self.current_token));

        AssignToVarArrNode(var, index, val)
    }
}
