use pest::iterators::Pair;

use crate::{
    generate::Generate,
    graphviz::{Node, ToNode},
    parser::Rule,
    symbol_table::SymbolTable,
};

mod cond_operand;
mod relop;

pub use cond_operand::*;
pub use relop::*;

/// Used for assembly jumps
pub static mut COND_COUNT: usize = 0;

#[derive(Debug)]
pub struct Cond {
    lhs: CondOperand,
    relop: RelOp,
    rhs: CondOperand,
}

impl Cond {
    pub fn try_execute(&self) -> Result<bool, ()> {
        match (&self.lhs, &self.rhs) {
            (CondOperand::Num(lhs), CondOperand::Num(rhs)) => Ok(self.relop.execute(*lhs, *rhs)),
            _ => Err(()),
        }
    }
}

impl From<Pair<'_, Rule>> for Cond {
    fn from(value: Pair<'_, Rule>) -> Self {
        debug_assert_eq!(value.as_rule(), Rule::cond);
        let mut inner = value.into_inner();

        let lhs = inner.next().unwrap();
        let lhs = if lhs.as_rule() == Rule::number {
            lhs.as_str().trim().parse::<i32>().unwrap().into()
        } else {
            lhs.as_str().trim().into()
        };

        let relop = inner.next().unwrap().into();

        let rhs = inner.next().unwrap();
        let rhs = if rhs.as_rule() == Rule::number {
            rhs.as_str().trim().parse::<i32>().unwrap().into()
        } else {
            rhs.as_str().trim().into()
        };

        Self { lhs, relop, rhs }
    }
}

impl ToNode for Cond {
    fn to_node(&self) -> crate::graphviz::Node {
        Node::new(&self.relop.to_string())
            .add(self.lhs.to_node())
            .add(self.rhs.to_node())
    }
}

impl Generate for Cond {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};

        let program = self
            .lhs
            .generate(program, symbol_table)
            .add(Mov(R8, Rbx.into()));
        let program = self
            .rhs
            .generate(program, symbol_table)
            .add(Mov(R9, Rbx.into()));

        let program = program.add(Cmp(R8, R9.into()));

        self.relop.generate(program, symbol_table)
    }
}
