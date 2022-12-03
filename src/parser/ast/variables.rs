use core::fmt;
use std::{
    fmt::{Display, Formatter},
    isize,
};

use either::Either;

use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
    Int,
    Float,
    String,
    Char,
    Bool,
    Type { name: String },
}

impl Display for VarType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayVarType {
    Int { init_num: isize },
    Float { init_num: isize },
    String { init_num: isize },
    Char { init_num: isize },
    Bool { init_num: isize },
    Type { name: String, init_num: isize },
}

impl ArrayVarType {
    pub fn to_var_type(&self) -> VarType {
        match self {
            ArrayVarType::Int { init_num: _ } => VarType::Int,
            ArrayVarType::Float { init_num: _ } => VarType::Float,
            ArrayVarType::String { init_num: _ } => VarType::String,
            ArrayVarType::Bool { init_num: _ } => VarType::Bool,
            ArrayVarType::Char { init_num: _ } => VarType::Char,
            ArrayVarType::Type {
                name: _,
                init_num: _,
            } => todo!(),
        }
    }

    pub fn get_init_num(&self) -> &isize {
        match self {
            ArrayVarType::Int { init_num } => init_num,
            ArrayVarType::Float { init_num } => init_num,
            ArrayVarType::String { init_num } => init_num,
            ArrayVarType::Bool { init_num } => init_num,
            ArrayVarType::Char { init_num } => init_num,
            ArrayVarType::Type {
                name: _,
                init_num: _,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarNode<'a> {
    pub name: String,
    pub ty: Either<VarType, ArrayVarType>,
    pub val: Either<Box<Node<'a>>, Vec<ArrElem<'a>>>,
    pub is_mut: bool,
}

impl<'a> VarNode<'a> {
    pub fn new(
        name: String,
        ty: Either<VarType, ArrayVarType>,
        val: Either<Box<Node<'a>>, Vec<ArrElem<'a>>>,
        is_mut: bool,
    ) -> Self {
        Self {
            name,
            ty,
            val,
            is_mut,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrElem<'a> {
    pub value: Box<Node<'a>>,
    pub index: isize,
}

impl<'a> ArrElem<'a> {
    pub fn new(value: Box<Node<'a>>, index: isize) -> Self {
        Self { value, index }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallVarNode<'a> {
    pub var_to_call: VarNode<'a>,
}

impl<'a> CallVarNode<'a> {
    pub fn new(var_to_call: VarNode<'a>) -> Self {
        Self { var_to_call }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallVarArrNode<'a> {
    pub var_to_call: CallVarNode<'a>,
    pub index_to_call: isize,
}

impl<'a> CallVarArrNode<'a> {
    pub fn new(var_to_call: CallVarNode<'a>, index_to_call: isize) -> Self {
        Self {
            var_to_call,
            index_to_call,
        }
    }
}
