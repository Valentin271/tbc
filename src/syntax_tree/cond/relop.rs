use std::fmt::Display;

use pest::iterators::Pair;

use super::COND_COUNT;
use crate::{generate::Generate, parser::Rule, symbol_table::SymbolTable};

/// All relational operator
#[derive(Debug)]
pub enum RelOp {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}

impl RelOp {
    pub fn execute(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            RelOp::Eq => lhs == rhs,
            RelOp::Ne => lhs != rhs,
            RelOp::Ge => lhs >= rhs,
            RelOp::Gt => lhs > rhs,
            RelOp::Le => lhs <= rhs,
            RelOp::Lt => lhs < rhs,
        }
    }
}

impl Display for RelOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RelOp::Eq => "==",
                RelOp::Ne => "<>",
                RelOp::Ge => ">=",
                RelOp::Gt => ">",
                RelOp::Le => "<=",
                RelOp::Lt => "<",
            }
        )
    }
}

impl From<Pair<'_, Rule>> for RelOp {
    fn from(value: Pair<'_, Rule>) -> Self {
        let inner = value.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::eq => RelOp::Eq,
            Rule::ne => RelOp::Ne,
            Rule::ge => RelOp::Ge,
            Rule::gt => RelOp::Gt,
            Rule::le => RelOp::Le,
            Rule::lt => RelOp::Lt,
            rule => unreachable!("Expected relop, found {:?}", rule),
        }
    }
}

impl Generate for RelOp {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        _: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Memory, Mnemonic::*};

        let then_label: Memory = format!("then{}", unsafe {
            COND_COUNT += 1;
            COND_COUNT
        })
        .into();

        program.add(match self {
            RelOp::Eq => Je(then_label),
            RelOp::Ne => Jne(then_label),
            RelOp::Ge => Jge(then_label),
            RelOp::Gt => Jg(then_label),
            RelOp::Le => Jle(then_label),
            RelOp::Lt => Jl(then_label),
        })
    }
}
