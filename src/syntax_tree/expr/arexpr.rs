use std::fmt::Display;

use pest::{iterators::Pair, pratt_parser::PrattParser};

use crate::{
    generate::Generate,
    graphviz::{Node, ToNode},
    optimize::Optimize,
    parser::Rule,
    symbol_table::SymbolTable,
};

/// Arithmetic operators
#[derive(Debug)]
pub enum ArOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArOp {
    pub fn execute(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            ArOp::Add => lhs + rhs,
            ArOp::Sub => lhs - rhs,
            ArOp::Mul => lhs * rhs,
            ArOp::Div => lhs / rhs,
        }
    }
}

impl Display for ArOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArOp::Add => "+",
                ArOp::Sub => "-",
                ArOp::Mul => "*",
                ArOp::Div => "/",
            }
        )
    }
}

impl From<Pair<'_, Rule>> for ArOp {
    fn from(value: Pair<'_, Rule>) -> Self {
        match value.as_rule() {
            Rule::add => Self::Add,
            Rule::sub => Self::Sub,
            Rule::mul => Self::Mul,
            Rule::div => Self::Div,
            rule => unreachable!("Expected operator, found {:?}", rule),
        }
    }
}

impl Generate for ArOp {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        _: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};

        match self {
            ArOp::Add => program.add(Add(R8, R9.into())),
            ArOp::Sub => program.add(Sub(R8, R9.into())),
            ArOp::Mul => program.add(IMul(R8, R9.into())),
            ArOp::Div => program
                .add(Mov(Rax, R8.into()))
                .add(Xor(Rdx, Rdx.into()))
                .add(IDiv(R9))
                .add(Mov(R8, Rax.into())),
        }
    }
}

/// An arithmetic expression
#[derive(Debug)]
pub enum Arexpr {
    Num(i32),
    Ident(String),
    BinExpr {
        lhs: Box<Arexpr>,
        op: ArOp,
        rhs: Box<Arexpr>,
    },
}

impl Arexpr {
    pub fn binexpr(lhs: Arexpr, op: ArOp, rhs: Arexpr) -> Self {
        Self::BinExpr {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    pub fn from_pair(value: Pair<'_, Rule>, _symbol_table: &mut SymbolTable) -> Self {
        use pest::pratt_parser::{Assoc, Op};

        let pratt = PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left));

        pratt
            .map_primary(|operand| match operand.as_rule() {
                Rule::number => Self::Num(operand.as_str().trim().parse().unwrap()),
                Rule::arexpr => Arexpr::from_pair(operand, _symbol_table),
                Rule::ident => Self::Ident(operand.as_str().trim().into()),
                rule => unreachable!("Expected operand, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| Arexpr::binexpr(lhs, op.into(), rhs))
            .parse(value.into_inner())
    }
}

impl ToNode for Arexpr {
    fn to_node(&self) -> Node {
        match self {
            Arexpr::Num(n) => n.to_node(),
            Arexpr::Ident(name) => name.to_node(),
            Arexpr::BinExpr { lhs, op, rhs } => Node::new(&op.to_string())
                .add(lhs.to_node())
                .add(rhs.to_node()),
        }
    }
}

impl Generate for Arexpr {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};

        match self {
            Arexpr::Num(n) => program.add(Push((*n).into())),
            Arexpr::Ident(name) => symbol_table.access(name, program).add(Push(Rbx.into())),
            Arexpr::BinExpr { lhs, op, rhs } => {
                let program = lhs.generate(program, symbol_table);
                let program = rhs.generate(program, symbol_table);

                let program = program.add(Pop(R9)).add(Pop(R8));

                let program = op.generate(program, symbol_table);

                program.add(Push(R8.into()))
            }
        }
    }
}

impl Optimize for Arexpr {
    fn optimize(self) -> Self {
        match self {
            Self::BinExpr { lhs, op, rhs } => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (lhs, rhs) {
                    (Arexpr::Num(n1), Arexpr::Num(n2)) => Arexpr::Num(op.execute(n1, n2)),
                    (l, r) => Arexpr::binexpr(l, op, r),
                }
            }
            _ => self,
        }
    }
}
