pub(crate) mod ast;

use std::vec;

use either::Either;
use Either::{Left, Right};

use crate::error_handler::Error;
use crate::lexer::token::{Span, Token, TokenType};
use crate::parser::ast::declarations::AssignType;
use crate::parser::ast::functions::FunctionNode;
use crate::parser::ast::loops::WhileNode;
use crate::parser::ast::types::TypeNode;

use self::ast::declarations::{AssignNode, ConstDeclNode, VarDeclNode, VarType};
use self::ast::functions::{ArgNode, FunctionDeclNode, ReturnIfNode, ReturnNode, UseFunctionNode};
use self::ast::identifier::{ArrayAccessNode, DotNotationNode, IdentifierNode};
use self::ast::if_else::IfNode;
use self::ast::import_export::{ExportNode, ImportNode};
use self::ast::loops::{ForNode, LoopNode};
use self::ast::math_ops::{BinOpNode, OpNode, UnOpNode};
use self::ast::switch::{CaseNode, DefaultNode, SwitchNode};
use self::ast::types::{
    BoolNode, CharNode, FloatNode, NewTypeValueNode, NumberNode, StringNode, SupportTypeNode,
};
use self::ast::{BlockNode, ConditionNode, Node, Nodes, ParamNode};

pub struct Parser<'a> {
    pub file_name: String,
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
    pub lines: Vec<String>,
    pub types: Vec<SupportTypeNode>,
    pub functions: Vec<String>,
    pub variables: Vec<(String, String)>,
    pub ast: Vec<Node<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        Self {
            file_name,
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            lines,
            types: Vec::new(),
            functions: Vec::new(),
            variables: Vec::new(),
            ast: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Node<'a> = self.parse_list(self.current_token);

            if new_node != Node::new(Nodes::NullNode) {
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
                line: 0,
                column: 0,
            }
        }
    }

    fn parse_list(&mut self, current_token: Token<'a>) -> Node<'a> {
        let next_token: Token = self.peek();

        match current_token.token_type {
            TokenType::Number
            | TokenType::Float
            | TokenType::NegativeNumber
            | TokenType::NegativeFloat => {
                if next_token.token_type == TokenType::Plus
                    || next_token.token_type == TokenType::Minus
                    || next_token.token_type == TokenType::Multiply
                    || next_token.token_type == TokenType::Divide
                    || next_token.token_type == TokenType::Power
                    || next_token.token_type == TokenType::Modulo
                {
                    Node::new(Nodes::OpNode(self.parse_op(true, true)))
                } else if next_token.token_type == TokenType::PlusPlus
                    || next_token.token_type == TokenType::MinusMinus
                {
                    Node::new(Nodes::OpNode(self.parse_op(false, true)))
                } else {
                    if current_token.token_type == TokenType::Number {
                        Node::new(Nodes::NumberNode(self.parse_number()))
                    } else {
                        Node::new(Nodes::FloatNode(self.parse_float()))
                    }
                }
            }
            TokenType::String => Node::new(Nodes::StringNode(self.parse_string())),
            TokenType::Char => Node::new(Nodes::CharNode(self.parse_char())),
            TokenType::BoolType => Node::new(Nodes::BoolNode(self.parse_bool())),
            TokenType::Let => Node::new(Nodes::VarDeclNode(
                self.parse_var(false, false).left().unwrap(),
            )),
            TokenType::Var => Node::new(Nodes::VarDeclNode(
                self.parse_var(true, false).left().unwrap(),
            )),
            TokenType::Const => Node::new(Nodes::ConstDeclNode(
                self.parse_var(false, true).right().unwrap(),
            )),
            TokenType::Type => Node::new(Nodes::TypeNode(self.parse_type())),
            TokenType::If => Node::new(Nodes::IfNode(self.parse_if_else())),
            TokenType::While => Node::new(Nodes::WhileNode(self.parse_while())),
            TokenType::For => Node::new(Nodes::ForNode(self.parse_for())),
            TokenType::Loop => Node::new(Nodes::LoopNode(self.parse_loop())),
            TokenType::Func => Node::new(Nodes::FunctionNode(self.parse_function())),
            TokenType::Return => Node::new(Nodes::ReturnNode(self.parse_return())),
            TokenType::ReturnIf => Node::new(Nodes::ReturnIfNode(self.parse_return_if())),
            TokenType::Switch => Node::new(Nodes::SwitchNode(self.parse_switch())),
            TokenType::Identifier => match next_token.token_type {
                TokenType::TripleColon => Node::new(Nodes::FunctionNode(self.parse_function())),
                TokenType::DoubleColon => {
                    Node::new(Nodes::UseFunctionNode(self.parse_use_function(true)))
                }
                TokenType::OpenParen => {
                    Node::new(Nodes::UseFunctionNode(self.parse_use_function(false)))
                }
                TokenType::OpenBracket => {
                    Node::new(Nodes::ArrayAccessNode(self.parse_array_access()))
                }
                TokenType::Dot => Node::new(Nodes::DotNotationNode(self.parse_dot_notation())),
                TokenType::OpenBrace => {
                    Node::new(Nodes::NewTypeValueNode(self.parse_new_type_value()))
                }
                TokenType::Assign
                | TokenType::PlusAssign
                | TokenType::MinusAssign
                | TokenType::MultiplyAssign
                | TokenType::DivideAssign
                | TokenType::ModuloAssign
                | TokenType::PowerAssign => {
                    Node::new(Nodes::AssignNode(self.parse_assign_to_var()))
                }
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Multiply
                | TokenType::Divide
                | TokenType::Power
                | TokenType::Modulo => Node::new(Nodes::OpNode(self.parse_op(true, false))),
                TokenType::PlusPlus | TokenType::MinusMinus => {
                    Node::new(Nodes::OpNode(self.parse_op(false, false)))
                }
                _ => Node::new(Nodes::IdentifierNode(self.parse_identifier())),
            },
            TokenType::Import | TokenType::Export => {
                if self.current_token.token_type == TokenType::Import {
                    return Node::new(Nodes::ImportNode(
                        self.parse_import_export().left().unwrap(),
                    ));
                } else {
                    return Node::new(Nodes::ExportNode(
                        self.parse_import_export().right().unwrap(),
                    ));
                }
            }
            _ => Node::new(Nodes::NullNode),
        }
    }

    fn parse_assign(&mut self) -> AssignType {
        match self.current_token.token_type {
            TokenType::Assign => AssignType::Assign,
            TokenType::PlusAssign => AssignType::AddAssign,
            TokenType::MinusAssign => AssignType::SubAssign,
            TokenType::MultiplyAssign => AssignType::MulAssign,
            TokenType::DivideAssign => AssignType::DivAssign,
            TokenType::ModuloAssign => AssignType::ModAssign,
            TokenType::PowerAssign => AssignType::PowAssign,
            _ => AssignType::Error,
        }
    }

    fn parse_val_type(&mut self, val: &Box<Node>) -> VarType {
        match val.node {
            Nodes::NumberNode(_) => VarType::new("int".into(), false),
            Nodes::FloatNode(_) => VarType::new("float".into(), false),
            Nodes::BoolNode(_) => VarType::new("bool".into(), false),
            Nodes::StringNode(_) => VarType::new("string".into(), false),
            Nodes::CharNode(_) => VarType::new("char".into(), false),
            _ => {
                for arr_type in self.types.clone() {
                    if arr_type.name == self.current_token.slice.to_string() {
                        return VarType::new(arr_type.name, false);
                    } else {
                        return VarType::new("Error".into(), false);
                    }
                }

                return VarType::new("Error".into(), false);
            }
        }
    }

    // Types
    fn parse_number(&self) -> NumberNode<'a> {
        NumberNode::new(self.current_token.clone())
    }

    fn parse_float(&self) -> FloatNode<'a> {
        FloatNode::new(self.current_token.clone())
    }

    fn parse_string(&self) -> StringNode<'a> {
        let token: Token<'a> = self.current_token.clone();

        return StringNode::new(token, token.slice.len());
    }

    fn parse_char(&self) -> CharNode<'a> {
        CharNode::new(self.current_token.clone())
    }

    fn parse_bool(&self) -> BoolNode<'a> {
        BoolNode::new(self.current_token.clone())
    }

    fn parse_ty_list(&mut self, is_array: bool) -> VarType {
        match self.current_token.token_type {
            TokenType::IntType => VarType::new("int".into(), is_array),
            TokenType::FloatType => VarType::new("float".into(), is_array),
            TokenType::BoolType => VarType::new("bool".into(), is_array),
            TokenType::StringType => VarType::new("string".into(), is_array),
            TokenType::CharType => VarType::new("char".into(), is_array),
            _ => {
                for arr_type in self.types.clone() {
                    if arr_type.name == self.current_token.slice.to_string() {
                        return VarType::new(arr_type.name, false);
                    }
                }

                Error::new(
                    self.current_token.clone(),
                    self.lines
                        .clone()
                        .into_iter()
                        .nth(self.current_token.line - 1)
                        .unwrap(),
                    self.file_name.clone(),
                )
                .throw_ty_not_found();

                return VarType::new("".into(), false);
            }
        }
    }

    fn parse_ty(&mut self) -> VarType {
        let ty: VarType;

        if self.peek().token_type == TokenType::OpenBracket {
            ty = self.parse_ty_list(true);

            self.next();
            self.next();
            self.next();
        } else {
            ty = self.parse_ty_list(false);
            self.next();
        }

        ty
    }

    // Identifiers
    fn parse_identifier(&self) -> IdentifierNode<'a> {
        IdentifierNode::new(self.current_token.clone())
    }

    fn parse_array_access(&mut self) -> ArrayAccessNode<'a> {
        self.next();
        self.next();

        let arr_pos: NumberNode = self.parse_number();

        self.next();

        ArrayAccessNode::new(arr_pos)
    }

    fn parse_dot_notation(&mut self) -> DotNotationNode<'a> {
        self.next();
        self.next();

        let next_token: Box<Node<'a>> = Box::new(self.parse_list(self.current_token));

        DotNotationNode::new(next_token)
    }

    // Ops
    fn parse_op(&mut self, is_bin_op: bool, is_number: bool) -> OpNode<'a> {
        let mut op_vec: Vec<Either<BinOpNode<'a>, UnOpNode<'a>>> = vec![];
        let mut node: Either<BinOpNode<'a>, UnOpNode<'a>> = Left(BinOpNode::new_useless(
            self.current_token.line,
            self.current_token.column,
            self.current_token.span,
        ));

        loop {
            if self.current_token.token_type == TokenType::Semicolon {
                break;
            }

            if is_bin_op {
                node = Left(self.parse_bin_op(is_number, node.left().unwrap().clone()));
            } else {
                node = Right(self.parse_un_op(is_number));
            }

            if self.current_token.token_type != TokenType::Minus
                || self.current_token.token_type != TokenType::Plus
                || self.current_token.token_type != TokenType::Multiply
                || self.current_token.token_type != TokenType::Divide
                || self.current_token.token_type != TokenType::Modulo
                || self.current_token.token_type != TokenType::Power
            {
                self.next()
            }

            op_vec.push(node.clone());
        }

        OpNode::new(op_vec.last().unwrap().clone())
    }

    fn parse_bin_op(&mut self, is_number: bool, old_node: BinOpNode<'a>) -> BinOpNode<'a> {
        let left_node: Either<Either<NumberNode<'a>, IdentifierNode<'a>>, Box<BinOpNode<'a>>> =
            if old_node.op_token.token_type == TokenType::Null {
                if is_number {
                    Left(Left(self.parse_number()))
                } else {
                    Left(Right(self.parse_identifier()))
                }
            } else {
                self.back();
                Right(Box::new(old_node.clone()))
            };

        self.next();

        let op_token: Token<'a> = self.current_token.clone();
        self.next();

        let right_node: Either<NumberNode<'a>, IdentifierNode<'a>> = if is_number {
            Left(self.parse_number())
        } else {
            Right(self.parse_identifier())
        };

        BinOpNode::new(left_node, op_token, right_node)
    }

    fn parse_un_op(&mut self, is_number: bool) -> UnOpNode<'a> {
        let left_node: Either<NumberNode<'a>, IdentifierNode<'a>> = if is_number {
            Left(self.parse_number())
        } else {
            Right(self.parse_identifier())
        };

        self.next();

        let op_token: Token = self.current_token;
        self.next();

        UnOpNode::new(left_node, op_token)
    }

    // Declarations
    fn parse_var(
        &mut self,
        is_mut: bool,
        is_const: bool,
    ) -> Either<VarDeclNode<'a>, ConstDeclNode<'a>> {
        self.next();

        let mut name: String = self.current_token.slice.into();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();
        self.next();

        let ty: VarType = self.parse_ty();
        self.variables.push((name.clone(), ty.to_string().clone()));

        let assign_token: AssignType = self.parse_assign();

        self.next();

        let mut value: Vec<Node<'a>> = vec![];

        for type_name in self.types.clone() {
            if &type_name.name == self.current_token.slice {
                self.back();
            }
        }

        value.append(&mut self.parse_value(ty.is_array));

        if is_const {
            return Right(ConstDeclNode::new(name, ty, assign_token, value));
        } else {
            return Left(VarDeclNode::new(name, ty, assign_token, is_mut, value));
        }
    }

    fn parse_assign_to_var(&mut self) -> AssignNode<'a> {
        let mut did_find: bool = false;
        let var: IdentifierNode<'a> = self.parse_identifier();
        let mut var_type: String = String::new();

        for var_arr in self.variables.clone() {
            if var_arr.0 == var.token.slice {
                var_type = var_arr.1.parse().unwrap();

                did_find = true;
                break;
            }
        }

        if !did_find {
            Error::new(
                var.token,
                self.lines
                    .clone()
                    .into_iter()
                    .nth(var.token.line - 1)
                    .unwrap(),
                self.file_name.clone(),
            )
            .throw_var_not_defined(var.token.slice);
        }

        did_find = false;

        self.next();

        let assign: AssignType = self.parse_assign();

        self.next();

        let val: Box<Node<'a>> = Box::new(self.parse_list(self.current_token));

        let val_type: VarType = self.parse_val_type(&val);

        for var_arr in self.variables.clone() {
            if var_arr.1 == val_type.to_string() {
                did_find = true;
                break;
            }
        }

        if !did_find {
            Error::new(
                var.token,
                self.lines
                    .clone()
                    .into_iter()
                    .nth(var.token.line - 1)
                    .unwrap(),
                self.file_name.clone(),
            )
            .throw_wrong_assign_type(var.token.slice, val_type.to_string(), var_type);
        }

        AssignNode::new(var, assign, val)
    }

    fn parse_type(&mut self) -> TypeNode {
        self.next();

        let name: String = self.current_token.slice.into();

        for type_ in &self.types {
            if type_.name == name {
                Error::new(
                    self.current_token,
                    self.lines
                        .clone()
                        .into_iter()
                        .nth(self.current_token.line - 1)
                        .unwrap(),
                    self.file_name.clone(),
                )
                .throw_type_name_already_used(name.clone());
            }
        }

        self.types.push(SupportTypeNode::new(name.clone(), vec![]));

        let mut fields: Vec<ParamNode> = vec![];

        self.next();
        self.next();

        while self.current_token.token_type != TokenType::CloseBrace {
            if self.current_token.token_type == TokenType::Comma {
                self.next();
            }

            if self.current_token.token_type == TokenType::Func {
                fields.push(self.parse_param(true));
            } else {
                fields.push(self.parse_param(false));
            }
        }

        let fields_str: Vec<String> = fields.clone().into_iter().map(|x| x.to_string()).collect();

        self.types.last_mut().unwrap().fields = fields_str;

        TypeNode::new(name, fields)
    }

    fn parse_new_type_value(&mut self) -> NewTypeValueNode<'a> {
        self.next();
        self.next();

        let mut params: Vec<Node> = vec![];

        while self.current_token.token_type != TokenType::CloseBrace {
            if self.current_token.token_type == TokenType::Comma {
                self.next();
            }
            params.push(self.parse_list(self.current_token));
            self.next();
        }

        NewTypeValueNode::new(params)
    }

    fn parse_value(&mut self, is_array: bool) -> Vec<Node<'a>> {
        let mut value: Vec<Node<'a>> = vec![];

        if is_array == false {
            while self.current_token.token_type != TokenType::Semicolon {
                value.push(self.parse_list(self.current_token));
                self.next();
            }
        } else {
            self.next();

            while self.current_token.token_type != TokenType::CloseBracket {
                if self.current_token.token_type == TokenType::Comma {
                    self.next()
                }

                value.push(self.parse_list(self.current_token));
                self.next();
            }
        }

        value
    }

    // Statements
    fn parse_condition(&mut self) -> ConditionNode<'a> {
        self.next();

        let left_node: Token<'a> = self.current_token.clone();
        self.next();

        let op_token: Token<'a> = self.current_token.clone();
        self.next();

        let right_node: Token<'a> = self.current_token.clone();

        ConditionNode::new(left_node, op_token, right_node)
    }

    fn parse_block(&mut self) -> BlockNode<'a> {
        let mut block_node: Vec<Node<'a>> = vec![];

        while self.current_token.token_type != TokenType::CloseBrace {
            let new_node: Node<'a> = self.parse_list(self.current_token);

            if new_node != Node::new(Nodes::NullNode) {
                block_node.push(new_node);
            }

            self.next();
        }

        BlockNode::new(block_node)
    }

    fn parse_if_else(&mut self) -> IfNode<'a> {
        let condition: ConditionNode<'a> = self.parse_condition();
        self.next();

        let if_block: BlockNode<'a> = self.parse_block();
        self.next();

        if self.current_token.token_type == TokenType::Else {
            self.next();

            let else_block: BlockNode<'a> = self.parse_block();

            return IfNode::new(condition, if_block, either::Left(else_block));
        }

        IfNode::new(condition, if_block, either::Right(()))
    }

    fn parse_while(&mut self) -> WhileNode<'a> {
        let condition: ConditionNode<'a> = self.parse_condition();
        self.next();

        let while_block: BlockNode<'a> = self.parse_block();

        WhileNode::new(condition, while_block)
    }

    fn parse_for(&mut self) -> ForNode<'a> {
        let condition: ConditionNode<'a> = self.parse_condition();
        self.next();

        let mut next_block: Either<UnOpNode<'a>, ()> = Either::Right(());

        if self.current_token.token_type == TokenType::Next {
            self.next();
            next_block = Either::Left(self.parse_un_op(false));
        }

        let for_block: BlockNode<'a> = self.parse_block();

        ForNode::new(condition, next_block, for_block)
    }

    fn parse_loop(&mut self) -> LoopNode<'a> {
        self.next();
        let loop_block: BlockNode<'a> = self.parse_block();
        LoopNode::new(loop_block)
    }

    // Switch
    fn parse_switch(&mut self) -> SwitchNode<'a> {
        self.next();

        let mut cases: Vec<CaseNode<'a>> = vec![];

        let mut default_node: DefaultNode<'a> = DefaultNode::new(BlockNode::new(vec![]));

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
            SwitchNode::new(cases, Left(default_node))
        } else {
            SwitchNode::new(cases, Right(()))
        }
    }

    fn parse_case(&mut self) -> CaseNode<'a> {
        let condition: ConditionNode<'a> = self.parse_condition();
        self.next();

        let case_block: BlockNode<'a> = self.parse_block();

        CaseNode::new(condition, case_block)
    }

    fn parse_default(&mut self) -> DefaultNode<'a> {
        let case_block: BlockNode<'a> = self.parse_block();

        DefaultNode::new(case_block)
    }

    // Functions
    fn parse_function_decl(&mut self) -> FunctionDeclNode {
        self.next();
        if self.current_token.token_type == TokenType::TripleColon {
            self.next();
        }

        let mut name: String = self.current_token.slice.into();
        self.functions.push(name.clone());

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();
        self.next();

        let mut params: Vec<ParamNode> = vec![];

        if self.current_token.token_type != TokenType::CloseParen {
            while self.current_token.token_type != TokenType::Colon {
                if self.current_token.token_type == TokenType::Func {
                    params.push(self.parse_param(true));
                } else {
                    params.push(self.parse_param(false));
                }
            }

            self.next();
        } else {
            self.next();
            self.next();
        }

        let ret_ty: VarType = self.parse_ty();

        self.next();

        FunctionDeclNode::new(name, params, ret_ty)
    }

    fn parse_function(&mut self) -> FunctionNode<'a> {
        let func_details: FunctionDeclNode = self.parse_function_decl();

        let function_block: BlockNode<'a> = self.parse_block();

        FunctionNode::new(func_details, function_block)
    }

    fn parse_use_function(&mut self, is_from_type: bool) -> UseFunctionNode<'a> {
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

        UseFunctionNode::new(name, args)
    }

    fn parse_return(&mut self) -> ReturnNode<'a> {
        self.next();

        let return_value: Box<Node<'a>> = Box::new(self.parse_list(self.current_token));

        ReturnNode::new(return_value)
    }

    fn parse_return_if(&mut self) -> ReturnIfNode<'a> {
        let condition: ConditionNode<'a> = self.parse_condition();
        self.next();

        let return_value: Box<Node<'a>> = Box::new(self.parse_list(self.current_token));

        ReturnIfNode::new(condition, return_value)
    }

    // Import & Export
    fn parse_take_ies(&mut self) -> Either<Box<Node<'a>>, Vec<Node<'a>>> {
        let args: Either<Box<Node<'a>>, Vec<Node<'a>>>;

        if self.current_token.token_type == TokenType::OpenBrace {
            let mut temp_vec: Vec<Node<'a>> = vec![];
            self.next();

            while self.current_token.token_type != TokenType::CloseBrace {
                if self.current_token.token_type == TokenType::Comma {
                    self.next()
                }

                temp_vec.push(self.parse_list(self.current_token));

                self.next();
            }

            args = Either::Right(temp_vec);
        } else {
            args = Either::Left(Box::new(self.parse_list(self.current_token)));
        }

        self.next();

        args
    }

    fn parse_import_export(&mut self) -> Either<ImportNode<'a>, ExportNode<'a>> {
        if self.current_token.token_type == TokenType::Export {
            self.next();

            let items: Either<Box<Node<'a>>, Vec<Node<'a>>> = self.parse_take_ies();

            return Right(ExportNode::new(items));
        }
        self.next();

        let import: Either<Box<Node<'a>>, Vec<Node<'a>>> = self.parse_take_ies();

        self.next();

        let from: Box<Node<'a>> = Box::new(self.parse_list(self.current_token));

        Left(ImportNode::new(import, from))
    }

    // Params
    fn parse_param(&mut self, is_func: bool) -> ParamNode {
        if is_func == true {
            let func_details: FunctionDeclNode = self.parse_function_decl();

            return ParamNode::new(func_details.name.clone(), Left(func_details));
        }
        let mut name: String = self.current_token.slice.into();

        self.next();

        if name.chars().next().unwrap().is_numeric() {
            name = "Error".to_string();
        }

        self.next();

        let ty: Either<FunctionDeclNode, VarType> = Either::Right(self.parse_ty());

        self.next();

        ParamNode::new(name, ty)
    }

    fn parse_arg(&mut self) -> Box<ArgNode<'a>> {
        let value: Node<'a> = self.parse_list(self.current_token);

        Box::new(ArgNode::new(value))
    }
}
