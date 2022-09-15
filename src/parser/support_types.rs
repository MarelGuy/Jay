use super::ast::{declarations::VarType, Node};

#[derive(Clone)]
pub struct VarSupportType<'a> {
    pub name: &'a str,
    pub ty: VarType,
    pub val: Vec<Node<'a>>,
}

impl<'a> VarSupportType<'a> {
    pub fn new(name: &'a str, ty: VarType, val: Vec<Node<'a>>) -> Self {
        Self { name, ty, val }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SupportTypeNode {
    pub name: String,
    pub fields: Vec<String>,
}

impl SupportTypeNode {
    pub fn new(name: String, fields: Vec<String>) -> Self {
        Self { name, fields }
    }
}
