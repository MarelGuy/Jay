use either::Either::{self, Left, Right};
use std::vec;

use crate::lexer::token::{Token, TokenType};
use crate::{error_handler::Error, lexer::token::Span};

use self::ast::functions::{
    ArgNode, CallFuncNode, FunctionNode, /*ReturnIfNode,*/ ReturnNode, ScopeNode,
};
use self::ast::primitive_node::TypeNode;
use self::ast::variables::AssignToVarArrNode;
use self::ast::{
    primitive_node::PrimitiveTypeNode,
    variables::{
        ArrElem, ArrayVarType, AssignToVarNode, CallVarArrNode, CallVarNode, VarNode, VarType,
    },
    Nodes,
};

pub(crate) mod ast;
mod math;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    file_name: String,
    lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Nodes<'a>>,

    use_local_scope: bool,

    error_handler: Error<'a>,

    current_scope: ScopeNode<'a>,
    global_scope: ScopeNode<'a>,
}

impl<'a> Parser<'a> {
    // * Main functions

    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        let init_tok: Token = token_stream[0];

        Self {
            file_name: file_name.clone(),
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],

            use_local_scope: false,

            error_handler: Error::new(init_tok, "".into(), file_name),

            current_scope: ScopeNode::new(vec![], vec![], vec![]),
            global_scope: ScopeNode::new(vec![], vec![], vec![]),
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Nodes<'a> = self.parse_list(self.current_token);

            self.ast.push(new_node);
        }
    }

    fn get_line(&self, line: usize) -> &str {
        &self.lines[line]
    }

    fn update_error_handler(&mut self) {
        self.error_handler.token = self.current_token;
        self.error_handler.line_string = self.get_line(self.current_token.line).into();
    }

    // * Flow functions

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
            self.token_stream[self.tok_i]
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

    // * Type functions

    fn parse_ty(&mut self) -> Either<VarType, ArrayVarType> {
        if self.peek().token_type == TokenType::OpenBracket {
            let tmp: ArrayVarType = self.get_array_ty().unwrap();

            self.next();

            Right(tmp)
        } else {
            Left(self.get_ty().unwrap())
        }
    }

    fn get_ty(&mut self) -> Option<VarType> {
        match self.current_token.token_type {
            TokenType::IntType => Some(VarType::Int),
            TokenType::FloatType => Some(VarType::Float),
            TokenType::StringType => Some(VarType::String),
            TokenType::BoolType => Some(VarType::Bool),
            TokenType::CharType => Some(VarType::Char),
            _ => {
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

                None
            }
        }
    }

    fn get_ty_from_val(&mut self, token: Token) -> Option<VarType> {
        match token.token_type {
            TokenType::Number | TokenType::NegativeNumber => Some(VarType::Int),
            TokenType::Float | TokenType::NegativeFloat => Some(VarType::Float),
            TokenType::String => Some(VarType::String),
            TokenType::Bool => Some(VarType::Bool),
            TokenType::Char => Some(VarType::Char),
            TokenType::Identifier => Some(if self.peek().token_type == TokenType::OpenParen {
                let found_node_idx: (Result<usize, usize>, i8) =
                    self.search_node(String::from(token.slice), true, 1);

                let found_node: FunctionNode<'a> = if found_node_idx.1 == 0 {
                    self.global_scope.func_vec[found_node_idx.0.unwrap()].clone()
                } else {
                    self.current_scope.func_vec[found_node_idx.0.unwrap()].clone()
                };

                found_node.ret_ty.unwrap_left()
            } else {
                let found_node_idx: (Result<usize, usize>, i8) =
                    self.search_node(String::from(token.slice), true, 0);

                let found_node: VarNode<'a> = if found_node_idx.1 == 0 {
                    self.global_scope.var_vec[found_node_idx.0.unwrap()].clone()
                } else {
                    self.current_scope.var_vec[found_node_idx.0.unwrap()].clone()
                };

                found_node.1.unwrap_left()
            }),
            _ => {
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

                None
            }
        }
    }

    fn get_array_ty(&mut self) -> Option<ArrayVarType> {
        let type_token: Token = self.current_token;

        self.next();
        self.next();

        let init_num: isize = self.current_token.slice.parse::<isize>().unwrap();

        match type_token.token_type {
            TokenType::IntType => Some(ArrayVarType::Int { init_num }),
            TokenType::FloatType => Some(ArrayVarType::Float { init_num }),
            TokenType::StringType => Some(ArrayVarType::String { init_num }),
            TokenType::BoolType => Some(ArrayVarType::Bool { init_num }),
            TokenType::CharType => Some(ArrayVarType::Char { init_num }),
            _ => {
                self.update_error_handler();
                self.error_handler.throw_ty_not_found();

                None
            }
        }
    }

    // * Search vec function

    fn search_node(
        &mut self,
        string_to_search: String,
        need_node: bool,
        vec_to_search: i8,
    ) -> (Result<usize, usize>, i8) {
        let mut found_where: i8 = 0;
        let mut node: Result<usize, usize>;
        match vec_to_search {
            0 => {
                node = self
                    .global_scope
                    .var_vec
                    .clone()
                    .into_iter()
                    .map(|x| -> String { x.0 })
                    .collect::<Vec<String>>()
                    .binary_search(&string_to_search);

                if self.use_local_scope && node.is_err() {
                    found_where = 1;

                    node = self
                        .current_scope
                        .var_vec
                        .clone()
                        .into_iter()
                        .map(|x| -> String { x.0 })
                        .collect::<Vec<String>>()
                        .binary_search(&string_to_search);
                }

                if need_node && node.is_err() {
                    self.update_error_handler();
                    self.error_handler.throw_name_not_defined(0)
                }
            }
            1 => {
                node = self
                    .global_scope
                    .func_vec
                    .clone()
                    .into_iter()
                    .map(|x| -> String { x.name })
                    .collect::<Vec<String>>()
                    .binary_search(&string_to_search);

                if self.use_local_scope && node.is_err() {
                    found_where = 1;

                    node = self
                        .current_scope
                        .func_vec
                        .clone()
                        .into_iter()
                        .map(|x| -> String { x.name })
                        .collect::<Vec<String>>()
                        .binary_search(&string_to_search);
                }

                if need_node && node.is_err() {
                    self.update_error_handler();
                    self.error_handler.throw_name_not_defined(1);
                }
            }
            _ => todo!(),
        }

        (node, found_where)
    }

    // * Parser

    fn parse_list(&mut self, token: Token<'a>) -> Nodes<'a> {
        match token.token_type {
            TokenType::Semicolon => Nodes::Eol,
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

                    Nodes::ProcessedMathNode(math::process_math_node(
                        tok_stream,
                        self.get_line(self.current_token.line).into(),
                        self.file_name.clone(),
                    ))
                } else {
                    if self.peek().token_type == TokenType::OpenParen {
                        let id: usize = self
                            .search_node(self.current_token.slice.into(), true, 1)
                            .0
                            .unwrap();

                        self.next();
                        self.next();

                        let mut args_vec: Vec<Nodes> = vec![];

                        while self.current_token.token_type != TokenType::CloseParen {
                            args_vec.push(self.parse_list(self.current_token));

                            self.next();

                            if self.current_token.token_type == TokenType::Comma {
                                self.next();
                            }
                        }

                        self.next();

                        return Nodes::CallFuncNode(CallFuncNode::new(id, args_vec));
                    }
                    self.search_node(self.current_token.slice.into(), false, 0)
                        .0
                        .is_ok()
                        .then(|| {
                            let is_var_node: bool;

                            let mut call_var_node: Nodes =
                                if self.peek().token_type == TokenType::OpenBracket {
                                    is_var_node = false;
                                    Nodes::CallVarArrNode(self.parse_call_var_arr())
                                } else {
                                    is_var_node = true;
                                    Nodes::CallVarNode(self.parse_call_var())
                                };

                            if self.peek().token_type == TokenType::Assign {
                                if is_var_node {
                                    call_var_node = Nodes::AssignToVarNode(
                                        self.parse_assign_to_var(call_var_node),
                                    );
                                } else {
                                    call_var_node = Nodes::AssignToVarArrNode(
                                        self.parse_assign_to_var_arr(call_var_node),
                                    );
                                }
                            }

                            call_var_node
                        })
                        .unwrap_or_else(|| {
                            Nodes::PrimitiveTypeNode(self.parse_primitive_type_node())
                        })
                }
            }
            TokenType::Let | TokenType::Var | TokenType::Const => Nodes::VarNode(self.parse_var()),
            TokenType::Func => Nodes::FunctionNode(self.parse_function()),
            TokenType::Return => Nodes::ReturnNode(self.parse_return()),
            TokenType::Type => Nodes::TypeNode(self.parse_type()),
            _ => {
                self.update_error_handler();
                self.error_handler.throw_unkown_token();
                Nodes::NullNode
            }
        }
    }

    fn parse_primitive_type_node(&mut self) -> PrimitiveTypeNode<'a> {
        PrimitiveTypeNode(self.current_token)
    }

    // * Variables

    fn parse_var(&mut self) -> VarNode<'a> {
        let is_mut: bool = self.current_token.token_type == TokenType::Var;

        self.next();

        let name: String = self.current_token.slice.into();

        if name.chars().next().unwrap().is_numeric() {
            self.update_error_handler();
            self.error_handler.throw_cant_start_var_num();
        };

        self.search_node(name.clone(), false, 0).0.is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_name_already_used(0);
        });

        self.next();
        self.next();

        let ty: Either<VarType, ArrayVarType> = self.parse_ty();

        self.next();
        self.next();

        if self.current_token.token_type == TokenType::OpenBracket {
            self.next();

            let mut index: isize = 0;
            let mut value: Vec<ArrElem<'a>> = vec![];

            loop {
                let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

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

            if self.use_local_scope {
                self.current_scope.var_vec.push(new_node.clone());
            } else {
                self.global_scope.var_vec.push(new_node.clone());
            }

            return new_node;
        }

        let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

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

        self.current_scope.var_vec.push(new_node.clone());

        new_node
    }

    fn parse_call_var(&mut self) -> CallVarNode<'a> {
        CallVarNode(
            self.current_scope.var_vec.clone()[self
                .search_node(self.current_token.slice.into(), true, 0)
                .0
                .unwrap()]
            .clone(),
        )
    }

    fn parse_index(&mut self) -> isize {
        if self.current_token.token_type == TokenType::Identifier {
            let node_to_parse: Box<Nodes<'a>> = if self.peek().token_type == TokenType::OpenBracket
            {
                // * Don't delete this variable.
                let idk0: CallVarArrNode = self.parse_call_var_arr();

                idk0.0 .0 .2.unwrap_right()[idk0.1 as usize].0.clone()
            } else {
                self.parse_call_var().0 .2.unwrap_left()
            };

            let unpacked_node: Token = node_to_parse.get_primitive().unwrap();

            if unpacked_node.token_type != TokenType::Number {
                let val_ty: VarType = self.get_ty_from_val(unpacked_node).unwrap();

                self.update_error_handler();
                self.error_handler
                    .throw_cant_use_val_in_arr_call(val_ty.to_string());
            }

            unpacked_node.slice.parse().unwrap()
        } else {
            if self.current_token.token_type != TokenType::Number {
                let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

                self.update_error_handler();
                self.error_handler
                    .throw_cant_use_val_in_arr_call(val_ty.to_string());
            }

            self.current_token.slice.parse().unwrap()
        }
    }

    fn parse_call_var_arr(&mut self) -> CallVarArrNode<'a> {
        let var_to_call: CallVarNode = self.parse_call_var();

        self.next();
        self.next();

        let index_to_call: isize = self.parse_index();

        self.next();

        while self.current_token.token_type == TokenType::CloseBracket {
            self.next();
        }

        self.back();

        if index_to_call < 0
            || var_to_call.0.clone().2.unwrap_right().last().unwrap().1 < index_to_call
        {
            self.update_error_handler();
            self.error_handler
                .throw_cant_use_num_array(var_to_call.0 .0.as_str(), index_to_call);
        }

        CallVarArrNode(var_to_call, index_to_call)
    }

    fn parse_assign_to_var(&mut self, var_to_assign: Nodes<'a>) -> AssignToVarNode<'a> {
        self.next();
        self.next();

        let var: CallVarNode<'a> = var_to_assign.get_call_var_node().unwrap();

        let var_ty: VarType = var.0 .1.clone().unwrap_left();
        let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

        if var_ty != val_ty {
            self.update_error_handler();
            self.error_handler.throw_wrong_assign_type(
                &var.0 .0,
                val_ty.to_string(),
                var_ty.to_string(),
            );
        }
        let val: Box<Nodes<'a>> = Box::new(self.parse_list(self.current_token));

        AssignToVarNode(var, val)
    }

    fn parse_assign_to_var_arr(&mut self, var_to_assign: Nodes<'a>) -> AssignToVarArrNode<'a> {
        self.back();
        self.back();

        let var: CallVarArrNode<'a> = var_to_assign.get_call_var_arr_node().unwrap();
        let index: isize = self.current_token.slice.parse().unwrap();

        for _ in 0..4 {
            self.next();
        }

        let var_ty: VarType = var.0 .0 .1.clone().unwrap_right().to_var_type();
        let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

        if var_ty != val_ty {
            self.update_error_handler();
            self.error_handler.throw_wrong_assign_type(
                &var.0 .0 .0,
                val_ty.to_string(),
                var_ty.to_string(),
            );
        }

        let val: Box<Nodes<'a>> = Box::new(self.parse_list(self.current_token));

        AssignToVarArrNode(var, index, val)
    }

    // * Functions

    // TODO: Error handling
    fn parse_function(&mut self) -> FunctionNode<'a> {
        self.next();

        let name: String = self.current_token.slice.into();

        self.search_node(name.clone(), false, 1).0.is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_name_already_used(1);
        });

        self.next();

        let mut args: Vec<ArgNode> = vec![];
        let mut args_name: Vec<String> = vec![];

        while self.current_token.token_type != TokenType::CloseParen {
            self.next();

            let arg: ArgNode = self.parse_func_arg(&mut args_name);

            args.push(arg.clone());
        }

        self.next();
        self.next();

        let ret_ty: Either<VarType, ArrayVarType> = self.parse_ty();

        self.next();
        self.next();

        self.use_local_scope = true;
        let scope: ScopeNode = self.parse_scope();
        self.use_local_scope = false;

        let new_node: FunctionNode = FunctionNode::new(name, args, ret_ty, scope);

        if self.use_local_scope {
            self.current_scope.func_vec.push(new_node.clone());
        } else {
            self.global_scope.func_vec.push(new_node.clone());
        }

        new_node
    }

    fn parse_scope(&mut self) -> ScopeNode<'a> {
        self.current_scope = ScopeNode::new(vec![], vec![], vec![]);

        while self.current_token.token_type != TokenType::CloseBrace {
            let node: Nodes<'a> = self.parse_list(self.current_token);

            self.current_scope.scope.push(node);
            self.next();
        }

        self.current_scope.clone()
    }

    // TODO function as arguments
    fn parse_func_arg(&mut self, arg_vec: &mut Vec<String>) -> ArgNode {
        let name: String = self.current_token.slice.into();

        if !arg_vec
            .clone()
            .into_iter()
            .any(|arg_name: String| arg_name == name)
        {
            arg_vec.push(name.clone());
        } else {
            self.update_error_handler();
            self.error_handler.throw_arg_alreay_used(name.clone());
        }

        self.next();
        self.next();

        let ty: Either<VarType, ArrayVarType> = self.parse_ty();

        self.next();

        ArgNode::new(name, ty)
    }

    fn parse_return(&mut self) -> ReturnNode<'a> {
        self.next();

        let ret_val: Nodes<'a> = self.parse_list(self.current_token);

        ReturnNode::new(Box::new(ret_val))
    }

    // * Types

    fn parse_type(&mut self) -> TypeNode {
        self.next();

        let name: String = String::from(self.current_token.slice);

        let mut args_vec: Vec<ArgNode> = vec![];
        let mut args_vec_names: Vec<String> = vec![];

        self.next();
        loop {
            self.next();

            if self.current_token.token_type == TokenType::CloseBrace {
                break;
            }

            let arg: ArgNode = self.parse_func_arg(&mut args_vec_names);

            args_vec.push(arg.clone());
        }

        self.next();

        TypeNode::new(name, args_vec)
    }

    // TODO: do after if
    // fn parse_return_if(&mut self) -> ReturnIfNode<'a> {
    //     todo!()
    // }
}
