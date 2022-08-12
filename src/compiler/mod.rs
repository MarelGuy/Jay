use crate::parser::ast::{
    declarations::{AssignNode, ConstDeclNode, VarDeclNode},
    functions::{FunctionNode, ReturnIfNode, ReturnNode, UseFunctionNode},
    general::{Node, Nodes},
    identifier::{ArrayAccessNode, DotNotationNode, IdentifierNode},
    if_else::IfNode,
    import_export::{ExportNode, ImportNode},
    loops::{ForNode, LoopNode, WhileNode},
    math_ops::{BinOpNode, UnOpNode},
    switch::SwitchNode,
    types::{BoolNode, CharNode, NewTypeValueNode, NumberNode, StringNode, TypeNode},
};

pub struct Compiler<'a> {
    ast: Vec<Box<Node<'a>>>,
}

impl<'a> Compiler<'a> {
    pub fn compile(&self) {
        for node in &self.ast {
            self.visit_node(&*node);
        }
    }

    pub fn new(ast: Vec<Box<Node<'a>>>) -> Self {
        Self { ast }
    }

    pub fn visit_node(&self, node: &Node<'a>) {
        match &*node.node {
            Nodes::VarDeclNode(node) => self.visit_var_decl_node(node),
            Nodes::ConstDeclNode(node) => self.visit_const_decl_node(node),
            Nodes::AssignNode(node) => self.visit_assign_node(node),
            Nodes::IdentifierNode(node) => self.visit_identifier_node(node),
            Nodes::ArrayAccessNode(node) => self.visit_array_access_node(node),
            Nodes::DotNotationNode(node) => self.visit_dot_notation_node(node),
            Nodes::IfNode(node) => self.visit_if_node(node),
            Nodes::SwitchNode(node) => self.visit_switch_node(node),
            Nodes::BinOpNode(node) => self.visit_binop_node(node),
            Nodes::UnOpNode(node) => self.visit_unop_node(node),
            Nodes::NumberNode(node) => self.visit_number_node(node),
            Nodes::StringNode(node) => self.visit_string_node(node),
            Nodes::CharNode(node) => self.visit_char_node(node),
            Nodes::BoolNode(node) => self.visit_bool_node(node),
            Nodes::TypeNode(node) => self.visit_type_node(node),
            Nodes::NewTypeValueNode(node) => self.visit_new_type_value_node(node),
            Nodes::WhileNode(node) => self.visit_while_node(node),
            Nodes::ForNode(node) => self.visit_for_node(node),
            Nodes::LoopNode(node) => self.visit_loop_node(node),
            Nodes::FunctionNode(node) => self.visit_function_node(node),
            Nodes::UseFunctionNode(node) => self.visit_use_function_node(node),
            Nodes::ReturnNode(node) => self.visit_return_node(node),
            Nodes::ReturnIfNode(node) => self.visit_return_if_node(node),
            Nodes::ImportNode(node) => self.visit_import_node(node),
            Nodes::ExportNode(node) => self.visit_export_node(node),
            Nodes::NullNode => self.visit_null_node(&node.node),
        }
    }

    fn visit_var_decl_node(&self, node: &VarDeclNode) {
        println!("Found var decl node.")
    }

    fn visit_const_decl_node(&self, node: &ConstDeclNode) {
        println!("Found const decl node.")
    }

    fn visit_assign_node(&self, node: &AssignNode) {
        println!("Found assign node.")
    }

    fn visit_identifier_node(&self, node: &IdentifierNode) {
        println!("Found identifier node.")
    }

    fn visit_array_access_node(&self, node: &ArrayAccessNode) {
        println!("Found array access node.")
    }

    fn visit_dot_notation_node(&self, node: &DotNotationNode) {
        println!("Found dot notation node.")
    }

    fn visit_if_node(&self, node: &IfNode) {
        println!("Found if node.")
    }

    fn visit_switch_node(&self, node: &SwitchNode) {
        println!("Found switch node.")
    }

    fn visit_binop_node(&self, node: &BinOpNode) {
        println!("Found binop node.")
    }

    fn visit_unop_node(&self, node: &UnOpNode) {
        println!("Found unop node.")
    }

    fn visit_number_node(&self, node: &NumberNode) {
        println!("Found number node.")
    }

    fn visit_string_node(&self, node: &StringNode) {
        println!("Found string node.")
    }

    fn visit_char_node(&self, node: &CharNode) {
        println!("Found char node.")
    }

    fn visit_bool_node(&self, node: &BoolNode) {
        println!("Found bool node.")
    }

    fn visit_type_node(&self, node: &TypeNode) {
        println!("Found type node.")
    }

    fn visit_new_type_value_node(&self, node: &NewTypeValueNode) {
        println!("Found new type value node.")
    }

    fn visit_while_node(&self, node: &WhileNode) {
        println!("Found while node.")
    }

    fn visit_for_node(&self, node: &ForNode) {
        println!("Found for node.")
    }

    fn visit_loop_node(&self, node: &LoopNode) {
        println!("Found loop node.")
    }

    fn visit_function_node(&self, node: &FunctionNode) {
        println!("Found function node.")
    }

    fn visit_use_function_node(&self, node: &UseFunctionNode) {
        println!("Found use function node.")
    }

    fn visit_return_node(&self, node: &ReturnNode) {
        println!("Found return node.")
    }

    fn visit_return_if_node(&self, node: &ReturnIfNode) {
        println!("Found return if node.")
    }

    fn visit_import_node(&self, node: &ImportNode) {
        println!("Found import node.")
    }

    fn visit_export_node(&self, node: &ExportNode) {
        println!("Found export node.")
    }

    fn visit_null_node(&self, node: &Nodes) {
        println!("Found null node.")
    }
}
