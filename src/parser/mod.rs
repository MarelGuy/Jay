use either::Either::{self, Left, Right};
use std::vec;

use crate::lexer::token::{Token, TokenType};
use crate::{error_handler::Error, lexer::token::Span};

use self::ast::functions::{
    ArgNode, CallFuncNode, DefineFunctionNode, FunctionNode, /*ReturnIfNode,*/ ReturnNode,
    ScopeNode,
};
use self::ast::types::TypeNode;
use self::ast::variables::{AssignToVarArrNode, InitTypeNode, ValueNode};
use self::ast::{
    types::PrimitiveTypeNode,
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

    error_handler: Error<'a>,

    current_scope: ScopeNode<'a>,
}

impl<'a> Parser<'a> {
    // * Main functions

    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        let init_tok: Token<'a> = token_stream[0];

        Self {
            file_name: file_name.clone(),
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],

            error_handler: Error::new(init_tok, "".to_owned(), file_name),

            current_scope: ScopeNode::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next(1);

            let new_node: Nodes<'a> = self.parse_list(self.current_token);

            self.ast.push(new_node);
        }
    }

    fn get_line(&self, line: usize) -> &str {
        &self.lines[line]
    }

    fn update_error_handler(&mut self) {
        self.error_handler.token = self.current_token;
        self.error_handler.line_string = self.get_line(self.current_token.line).to_owned();
    }

    pub fn search_node(
        &mut self,
        string_to_search: String,
        need_node: bool,
        vec_to_search: u8,
    ) -> (Result<usize, usize>, bool) {
        let result = self
            .current_scope
            .search_node(string_to_search, vec_to_search);

        if need_node && result.1 {
            self.update_error_handler();
            self.error_handler.throw_name_not_defined(vec_to_search);
        }

        result
    }

    // * Flow functions

    fn back(&mut self) {
        self.tok_i -= 1;

        self.current_token = self.token_stream[self.tok_i];
    }

    fn next(&mut self, count: usize) {
        for _ in 0..count {
            if self.tok_i < self.token_stream.len() {
                self.current_token = self.token_stream[self.tok_i];
            }

            self.tok_i += 1;
        }
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

            self.next(1);

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
                self.search_node(self.current_token.slice.to_owned(), true, 2)
                    .0
                    .unwrap();

                Some(VarType::Type {
                    name: self.current_token.slice.to_owned(),
                })
            }
        }
    }

    fn get_ty_from_val(&mut self, token: Token<'a>) -> Option<VarType> {
        match token.token_type {
            TokenType::Number | TokenType::NegativeNumber => Some(VarType::Int),
            TokenType::Float | TokenType::NegativeFloat => Some(VarType::Float),
            TokenType::String => Some(VarType::String),
            TokenType::Bool => Some(VarType::Bool),
            TokenType::Char => Some(VarType::Char),
            TokenType::Identifier => Some({
                match self.peek().token_type {
                    TokenType::OpenParen => {
                        let found_node_idx: (Result<usize, usize>, bool) =
                            self.search_node(token.slice.to_owned(), true, 1);

                        self.current_scope.func_vec[found_node_idx.0.unwrap()]
                            .clone()
                            .define_node
                            .ret_ty?
                            .unwrap_left()
                    }
                    TokenType::OpenBrace => {
                        let found_node_idx: (Result<usize, usize>, bool) =
                            self.search_node(token.slice.to_owned(), true, 2);

                        let found_node: TypeNode =
                            self.current_scope.type_vec[found_node_idx.0.unwrap()].clone();

                        VarType::Type {
                            name: found_node.name,
                        }
                    }
                    _ => {
                        let found_node_idx: (Result<usize, usize>, bool) =
                            self.search_node(token.slice.to_owned(), true, 0);

                        self.current_scope.var_vec[found_node_idx.0.unwrap()]
                            .clone()
                            .1
                             .1
                            .unwrap_left()
                    }
                }
            }),
            _ => {
                self.update_error_handler();
                self.error_handler.throw_name_not_defined(2);

                None
            }
        }
    }

    fn get_array_ty(&mut self) -> Option<ArrayVarType> {
        let type_token: Token<'a> = self.current_token;
        let type_name: String = self.current_token.slice.to_owned();

        self.next(2);

        let init_num: isize = self.current_token.slice.parse::<isize>().unwrap();

        match type_token.token_type {
            TokenType::IntType => Some(ArrayVarType::Int { init_num }),
            TokenType::FloatType => Some(ArrayVarType::Float { init_num }),
            TokenType::StringType => Some(ArrayVarType::String { init_num }),
            TokenType::BoolType => Some(ArrayVarType::Bool { init_num }),
            TokenType::CharType => Some(ArrayVarType::Char { init_num }),
            _ => {
                self.search_node(type_name, true, 2).0.unwrap();

                Some(ArrayVarType::Type {
                    name: self.current_token.slice.to_owned(),
                    init_num,
                })
            }
        }
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
            | TokenType::NegativeNumber => match self.peek().token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply => {
                    let mut tok_stream: Vec<Token<'a>> = vec![];

                    loop {
                        tok_stream.push(self.current_token);

                        self.next(1);

                        if self.current_token.token_type == TokenType::Semicolon
                            || self.current_token.token_type == TokenType::Colon
                            || self.current_token.token_type == TokenType::CloseBracket
                        {
                            self.back();
                            break;
                        }
                    }

                    return Nodes::ProcessedMathNode(math::process_math_node(
                        tok_stream,
                        self.get_line(self.current_token.line).to_owned(),
                        self.file_name.clone(),
                    ));
                }
                TokenType::OpenParen => {
                    let id: usize = self
                        .search_node(self.current_token.slice.to_owned(), true, 1)
                        .0
                        .unwrap();

                    self.next(2);

                    let mut args_vec: Vec<Nodes<'a>> = vec![];

                    while self.current_token.token_type != TokenType::CloseParen {
                        args_vec.push(self.parse_list(self.current_token));

                        self.next(if self.peek().token_type == TokenType::Comma {
                            2
                        } else {
                            1
                        });
                    }

                    self.next(1);

                    return Nodes::CallFuncNode(CallFuncNode::new(id, args_vec));
                }
                _ => {
                    let vec_to_search: u8 = if self.peek().token_type == TokenType::OpenBrace {
                        2
                    } else {
                        0
                    };

                    let found_node: Result<usize, usize> = self
                        .search_node(self.current_token.slice.to_owned(), false, vec_to_search)
                        .0;

                    if found_node.is_err() {
                        return Nodes::PrimitiveTypeNode(self.parse_primitive_type_node());
                    }

                    match vec_to_search {
                        0 => {
                            let is_var_node: bool;

                            let mut call_var_node: Nodes<'a> =
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
                        }
                        2 => Nodes::InitTypeNode(self.parse_type_init()),
                        _ => Nodes::PrimitiveTypeNode(self.parse_primitive_type_node()),
                    }
                }
            },
            TokenType::Let | TokenType::Var | TokenType::Const => Nodes::VarNode(self.parse_var()),
            TokenType::Func => Nodes::FunctionNode(self.parse_function()),
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

    fn parse_value(&mut self, name: String, ty: Either<VarType, ArrayVarType>) -> ValueNode<'a> {
        if self.current_token.token_type == TokenType::OpenBracket {
            self.next(1);

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

                self.next(1);

                if self.current_token.token_type == TokenType::Comma {
                    index += 1;
                    self.next(1);
                } else if self.current_token.token_type == TokenType::CloseBracket {
                    break;
                }
            }

            ValueNode(Either::Right(value), ty)
        } else {
            let val_ty: VarType = self.get_ty_from_val(self.current_token).unwrap();

            if val_ty != ty.clone().unwrap_left() {
                self.update_error_handler();
                self.error_handler.throw_wrong_assign_type(
                    &name,
                    val_ty.to_string(),
                    ty.clone().unwrap_left().to_string(),
                );
            }
            ValueNode(
                Either::Left(Box::new(self.parse_list(self.current_token))),
                ty,
            )
        }
    }

    fn parse_var(&mut self) -> VarNode<'a> {
        let is_mut: bool = self.current_token.token_type == TokenType::Var;

        self.next(1);

        let name: String = self.current_token.slice.to_owned();

        if name.chars().next().unwrap().is_numeric() {
            self.update_error_handler();
            self.error_handler.throw_cant_start_var_num();
        };

        self.search_node(name.clone(), false, 0).0.is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_name_already_used(0);
        });

        self.next(2);

        let ty: Either<VarType, ArrayVarType> = self.parse_ty();

        self.next(2);

        let new_node: VarNode<'a> = VarNode(name.clone(), self.parse_value(name, ty), is_mut);

        self.current_scope.var_vec.push(new_node.clone());

        new_node
    }

    fn parse_call_var(&mut self) -> CallVarNode<'a> {
        let idk: (Result<usize, usize>, bool) =
            self.search_node(self.current_token.slice.to_owned(), true, 0);

        let a: CallVarNode = CallVarNode(self.current_scope.var_vec[idk.0.unwrap()].clone());

        a
        // CallVarNode(if self.use_local_scope {
        //     self.current_scope.var_vec.clone()[self
        //         .search_node(self.current_token.slice.to_owned(), true, 0)
        //         .0
        //         .unwrap()]
        //     .clone()
        // } else {
        //     self.global_scope.var_vec.clone()[self
        //         .search_node(self.current_token.slice.to_owned(), true, 0)
        //         .0
        //         .unwrap()]
        //     .clone()
        // })
    }

    fn parse_index(&mut self) -> isize {
        if self.current_token.token_type == TokenType::Identifier {
            let node_to_parse: Box<Nodes<'a>> = if self.peek().token_type == TokenType::OpenBracket
            {
                // * Don't delete this variable.
                let idk0: CallVarArrNode<'a> = self.parse_call_var_arr();

                idk0.0 .0 .1 .0.unwrap_right()[idk0.1 as usize].0.clone()
            } else {
                self.parse_call_var().0 .1 .0.unwrap_left()
            };

            let unpacked_node: Token<'a> = node_to_parse.get_primitive().unwrap();

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
        let var_to_call: CallVarNode<'a> = self.parse_call_var();

        self.next(2);

        let index_to_call: isize = self.parse_index();

        self.next(1);

        while self.current_token.token_type == TokenType::CloseBracket {
            self.next(1);
        }

        self.back();

        if index_to_call < 0
            || var_to_call.0.clone().1 .0.unwrap_right().last().unwrap().1 < index_to_call
        {
            self.update_error_handler();
            self.error_handler
                .throw_cant_use_num_array(var_to_call.0 .0.as_str(), index_to_call);
        }

        CallVarArrNode(var_to_call, index_to_call)
    }

    fn parse_assign_to_var(&mut self, var_to_assign: Nodes<'a>) -> AssignToVarNode<'a> {
        self.next(2);

        let var: CallVarNode<'a> = var_to_assign.get_call_var_node().unwrap();

        let var_ty: VarType = var.0 .1 .1.clone().unwrap_left();
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

        self.next(4);

        let var_ty: VarType = var.0 .0 .1 .1.clone().unwrap_right().to_var_type();
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
        self.next(1);

        let define_func_node: DefineFunctionNode = self.parse_define_function_node();

        let mut new_node: FunctionNode<'a> =
            FunctionNode::new(define_func_node.clone(), ScopeNode::new());

        self.current_scope.func_vec.push(new_node.clone());

        let scope: ScopeNode = self.parse_scope(define_func_node);

        new_node.scope = scope.clone();

        self.current_scope.func_vec.last_mut().unwrap().scope = scope;

        new_node
    }

    fn parse_define_function_node(&mut self) -> DefineFunctionNode {
        let name: String = self.current_token.slice.to_owned();

        self.search_node(name.clone(), false, 1).0.is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_name_already_used(1);
        });

        self.next(1);

        let mut args: Vec<ArgNode> = vec![];
        let mut args_name: Vec<String> = vec![];

        while self.current_token.token_type != TokenType::CloseParen {
            self.next(1);

            let arg: ArgNode = self.parse_func_arg(&mut args_name);

            args.push(arg.clone());
        }

        self.next(1);
        let ret_ty: Option<Either<VarType, ArrayVarType>> =
            if self.current_token.token_type == TokenType::Colon {
                self.next(1);
                let tmp: Option<Either<VarType, ArrayVarType>> = Some(self.parse_ty());
                self.next(1);

                tmp
            } else {
                None
            };
        self.next(1);

        DefineFunctionNode::new(name, args, ret_ty)
    }

    fn parse_scope(&mut self, func: DefineFunctionNode) -> ScopeNode<'a> {
        while self.current_token.token_type != TokenType::CloseBrace {
            if self.current_token.token_type == TokenType::Return {
                if func.clone().ret_ty.is_some() {
                    let return_node: Nodes =
                        Nodes::ReturnNode(self.parse_return(func.ret_ty.clone()));

                    self.current_scope.scope.push(return_node);
                } else {
                    self.update_error_handler();
                    self.error_handler
                        .throw_used_return_when_no_return(func.name.clone())
                }
            }

            let node: Nodes<'a> = self.parse_list(self.current_token);

            self.current_scope.scope.push(node);
            self.next(1);
        }

        self.current_scope.clone()
    }

    // TODO function as arguments
    fn parse_func_arg(&mut self, arg_vec: &mut Vec<String>) -> ArgNode {
        if self.current_token.token_type == TokenType::Func {
            let node: FunctionNode = self.parse_function();

            return ArgNode::new(node.define_node.name, node.define_node.ret_ty.unwrap());
        }

        let name: String = self.current_token.slice.to_owned();

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

        self.next(2);

        let ty: Either<VarType, ArrayVarType> = self.parse_ty();

        self.next(1);

        ArgNode::new(name, ty)
    }

    fn parse_return(&mut self, ret_ty: Option<Either<VarType, ArrayVarType>>) -> ReturnNode<'a> {
        self.next(1);

        let ret_val: ValueNode<'a> = self.parse_value("".to_owned(), ret_ty.unwrap());

        ReturnNode::new(ret_val)
    }

    // * Types

    fn parse_type(&mut self) -> TypeNode<'a> {
        self.next(1);

        let name: String = self.current_token.slice.to_owned();

        self.search_node(name.clone(), false, 2).0.is_ok().then(|| {
            self.update_error_handler();
            self.error_handler.throw_name_already_used(2);
        });

        let mut args_vec: Vec<Either<ArgNode, FunctionNode<'a>>> = vec![];
        let mut args_vec_names: Vec<String> = vec![];

        let mut node: TypeNode = TypeNode::new(name, vec![]);

        self.current_scope.type_vec.push(node.clone());

        self.next(1);

        loop {
            self.next(1);

            if self.current_token.token_type == TokenType::CloseBrace {
                break;
            }

            let arg: Either<ArgNode, FunctionNode<'a>> =
                if self.current_token.token_type == TokenType::Identifier {
                    Left(self.parse_func_arg(&mut args_vec_names))
                } else {
                    Right(self.parse_function())
                };

            args_vec.push(arg.clone());
            self.current_scope
                .type_vec
                .last_mut()
                .unwrap()
                .args
                .push(arg);
        }

        self.next(1);

        node.args = args_vec;

        node
    }

    fn parse_type_init(&mut self) -> InitTypeNode<'a> {
        let idx: (Result<usize, usize>, bool) =
            self.search_node(self.current_token.slice.to_owned(), true, 2);

        let found_node: TypeNode = self.current_scope.type_vec[idx.0.unwrap()].clone();

        self.next(2);

        let mut fields: Vec<ValueNode<'a>> = vec![];
        let mut i: usize = 0;

        while self.current_token.token_type != TokenType::CloseBrace {
            fields.push(self.parse_value(
                "".to_owned(),
                found_node.args[i].clone().unwrap_left().ty.clone(),
            ));

            self.next(if self.peek().token_type == TokenType::Comma {
                2
            } else {
                1
            });

            i += 1;
        }

        self.next(1);

        InitTypeNode::new(fields)
    }

    // TODO: do after if
    // fn parse_return_if(&mut self) -> ReturnIfNode<'a> {
    //     todo!()
    // }
}
