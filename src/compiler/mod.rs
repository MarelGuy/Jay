use std::error::Error;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::IntType,
    values::{IntValue, PointerValue},
};

use crate::parser::ast::{
    math::{Operation, Operator},
    Nodes,
};

pub struct Codegen<'a> {
    // pub(super) context: &'a Context,
    pub(super) module: Module<'a>,
    pub(super) builder: Builder<'a>,
    // pub(super) main: FunctionValue<'a>,
    pub(super) main_block: BasicBlock<'a>,
    ast: Vec<Nodes<'a>>,

    // Types
    i32_t: IntType<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(
        context: &'a Context,
        module: Module<'a>,
        builder: Builder<'a>,
        // main: FunctionValue<'a>,
        main_block: BasicBlock<'a>,
        ast: Vec<Nodes<'a>>,
    ) -> Self {
        Self {
            // context,
            module,
            builder,
            // main,
            ast,
            main_block,
            i32_t: context.i32_type(),
        }
    }

    pub fn compile(&self) {
        self.builder.position_at_end(self.main_block);

        for node in &self.ast {
            self.visit_node(*node);
        }
    }

    pub fn visit_node(&self, node: Nodes) {
        match node {
            Nodes::Op(_) => self.visit_op(&node).unwrap(),
            _ => panic!(),
        };
    }

    fn visit_op(&self, node: &Nodes) -> Result<(), Box<dyn Error>> {
        let op: Operation = Nodes::get_op(*node).unwrap();

        let lhs: u64 = op.lhs.val.parse::<u64>()?;
        let rhs: u64 = op.rhs.val.parse::<u64>()?;

        let store_lhs: IntValue<'_> = self.i32_t.const_int(lhs, false);
        let store_rhs: IntValue<'_> = self.i32_t.const_int(rhs, false);

        let add_res: IntValue<'_> = match op.op {
            Operator::Plus => self.builder.build_int_add(store_lhs, store_rhs, "result"),
            Operator::Minus => self.builder.build_int_sub(store_lhs, store_rhs, "result"),
            Operator::Multiply => self.builder.build_int_mul(store_lhs, store_rhs, "result"),
            Operator::Divide => self
                .builder
                .build_int_signed_div(store_lhs, store_rhs, "result"),
            Operator::Modulo => self
                .builder
                .build_int_signed_rem(store_lhs, store_rhs, "result"),
        }?;

        let result: PointerValue<'_> = self.builder.build_alloca(self.i32_t, "result")?;
        self.builder.build_store(result, add_res)?;

        Ok(())
    }
}
