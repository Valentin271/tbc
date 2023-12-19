use crate::{
    generate::Generate,
    graphviz::{Node, ToNode},
    symbol_table::SymbolTable,
};

/// The operand of a condition
#[derive(Debug)]
pub enum CondOperand {
    Num(i32),
    Ident(String),
}

impl From<i32> for CondOperand {
    fn from(value: i32) -> Self {
        Self::Num(value)
    }
}

impl From<&str> for CondOperand {
    fn from(value: &str) -> Self {
        Self::Ident(value.to_string())
    }
}

impl ToNode for CondOperand {
    fn to_node(&self) -> Node {
        match self {
            CondOperand::Num(n) => n.to_node(),
            CondOperand::Ident(ident) => ident.to_node(),
        }
    }
}

impl Generate for CondOperand {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};
        match self {
            CondOperand::Num(n) => program.add(Mov(Rbx, n.to_owned().into())),
            CondOperand::Ident(ident) => symbol_table.access(ident, program),
        }
    }
}
