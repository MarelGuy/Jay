use either::Either;

use super::{
    types::TypeNode,
    variables::{ArrayVarType, ValueNode, VarNode, VarType},
    Nodes,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DefineFunctionNode {
    pub name: String,
    args: Vec<ArgNode>,
    pub ret_ty: Option<Either<VarType, ArrayVarType>>,
}

impl DefineFunctionNode {
    pub fn new(
        name: String,
        args: Vec<ArgNode>,
        ret_ty: Option<Either<VarType, ArrayVarType>>,
    ) -> Self {
        Self { name, args, ret_ty }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode<'a> {
    pub define_node: DefineFunctionNode,
    pub scope: ScopeNode<'a>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(define_node: DefineFunctionNode, scope: ScopeNode<'a>) -> Self {
        Self { define_node, scope }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeNode<'a> {
    pub scope: Vec<Nodes<'a>>,
    pub var_vec: Vec<VarNode<'a>>,
    pub func_vec: Vec<FunctionNode<'a>>,
    pub type_vec: Vec<TypeNode<'a>>,
}

impl<'a> ScopeNode<'a> {
    pub fn new() -> Self {
        Self {
            scope: vec![],
            var_vec: vec![],
            func_vec: vec![],
            type_vec: vec![],
        }
    }

    pub fn search_node(
        &mut self,
        string_to_search: String,
        vec_to_search: u8,
    ) -> (Result<usize, usize>, bool) {
        let node: Result<usize, usize> = match vec_to_search {
            0 => self
                .var_vec
                .clone()
                .into_iter()
                .map(|x| -> String { x.0 })
                .collect::<Vec<String>>()
                .binary_search(&string_to_search),
            1 => self
                .func_vec
                .clone()
                .into_iter()
                .map(|x| -> String { x.define_node.name })
                .collect::<Vec<String>>()
                .binary_search(&string_to_search),
            2 => self
                .type_vec
                .clone()
                .into_iter()
                .map(|x| -> String { x.name })
                .collect::<Vec<String>>()
                .binary_search(&string_to_search),
            _ => todo!(),
        };

        (node, node.is_err())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArgNode {
    pub name: String,
    pub ty: Either<VarType, ArrayVarType>,
}

impl ArgNode {
    pub fn new(name: String, ty: Either<VarType, ArrayVarType>) -> Self {
        Self { name, ty }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallFuncNode<'a> {
    func_node: usize,
    args: Vec<Nodes<'a>>,
}

impl<'a> CallFuncNode<'a> {
    pub fn new(func_node: usize, args: Vec<Nodes<'a>>) -> Self {
        Self { func_node, args }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode<'a> {
    ret_val: ValueNode<'a>,
}

impl<'a> ReturnNode<'a> {
    pub fn new(ret_val: ValueNode<'a>) -> Self {
        Self { ret_val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnIfNode<'a> {
    ret_val: Box<Nodes<'a>>,
}
