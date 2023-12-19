use pest::iterators::Pair;

use super::{Arexpr, Cond, Expr};
use crate::{
    error::SyntaxError,
    generate::Generate,
    graphviz::{Node, ToNode},
    optimize::Optimize,
    parser::Rule,
    symbol_table::{SymbolTable, Type},
    syntax_tree::COND_COUNT,
};

#[derive(Debug)]
pub enum Stmt {
    End,
    Goto(u32),
    If {
        cond: Cond,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    Input(String),
    Let(String, Arexpr),
    Print(Expr),
    /// This is mainly useful for optimization purposes
    NoOp,
}

impl Stmt {
    pub fn from_pair(
        value: Pair<'_, Rule>,
        symbol_table: &mut SymbolTable,
    ) -> Result<Self, SyntaxError> {
        let mut pairs = value.into_inner();
        let stmt = pairs.next().unwrap();

        Ok(match stmt.as_rule() {
            Rule::end | Rule::r#return => Stmt::End,
            Rule::goto | Rule::gosub => {
                let line = pairs.next().unwrap();
                Stmt::Goto(line.as_str().parse()?)
            }
            Rule::r#if => Stmt::If {
                cond: pairs.next().unwrap().into(),
                then: Box::new(Self::from_pair(pairs.next().unwrap(), symbol_table)?),
                els: {
                    if pairs.next().is_some() {
                        Some(Box::new(Self::from_pair(
                            pairs.next().unwrap(),
                            symbol_table,
                        )?))
                    } else {
                        None
                    }
                },
            },
            Rule::input => {
                let ident = pairs.next().unwrap().as_str();
                symbol_table.insert(ident, Type::Int);
                Stmt::Input(ident.into())
            }
            Rule::r#let => {
                let ident = pairs.next().unwrap();
                let expr = pairs.next().unwrap();

                let ident = ident.as_str().trim().to_string();
                let expr = Arexpr::from_pair(expr, symbol_table);

                symbol_table.insert(&ident, Type::Int);

                Stmt::Let(ident, expr)
            }
            Rule::print => Stmt::Print(Expr::from_pair(pairs.next().unwrap(), symbol_table)),
            rule => unimplemented!("Unknown statement {:?}", rule),
        })
    }
}

impl ToNode for Stmt {
    fn to_node(&self) -> Node {
        match self {
            Stmt::End => Node::new("end"),
            Stmt::Goto(line) => Node::new("goto").add(line.to_node()),
            Stmt::If { cond, then, els } => {
                let node = Node::new("if").add(cond.to_node()).add(then.to_node());
                if let Some(els) = els {
                    node.add(els.to_node())
                } else {
                    node
                }
            }
            Stmt::Input(ident) => Node::new("input").add(ident.to_node()),
            Stmt::Let(ident, value) => Node::new("let").add(ident.to_node()).add(value.to_node()),
            Stmt::Print(expr) => Node::new("print").add(expr.to_node()),
            Stmt::NoOp => Node::new(""),
        }
    }
}

impl Generate for Stmt {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Memory, Mnemonic::*, Register::*};

        static mut LITERAL_COUNT: usize = 0;

        match self {
            Stmt::End => program.add(Jmp("exit".into())),
            Stmt::Goto(line) => program.add(Jmp(Memory::from(format!("line{line}")))),
            Stmt::If { cond, then, els } => {
                let mut program = cond.generate(program, symbol_table);

                let count = unsafe { COND_COUNT };
                let endif_label = format!("fi{}", count);

                // else
                if let Some(els) = els {
                    program = program.label(&format!("else{}", count));
                    program = els
                        .generate(program, symbol_table)
                        .add(Jmp(endif_label.clone().into()));
                } else {
                    program = program.add(Jmp(endif_label.clone().into()))
                }

                // then
                program = program.label(&format!("then{}", count));
                program = then.generate(program, symbol_table);

                program.label(&endif_label)
            }
            Stmt::Input(ident) => {
                let end_addr = symbol_table.get(ident).unwrap().end_addr() as i32;

                let program = program
                    .add(Mov(Rsi, R15.into()))
                    .add(Sub(Rsi, end_addr.into()))
                    .add(Mov(Rdx, 8.into()))
                    .add(Call("read".into()));

                let program = symbol_table.access(ident, program).add(Sub(Rbx, 48.into()));

                symbol_table.write(ident, Rbx.into(), program)
            }
            Stmt::Let(ident, arexpr) => {
                let program = arexpr.generate(program, symbol_table).add(Pop(Rbx));
                symbol_table.write(ident, Rbx.into(), program)
            }
            Stmt::Print(Expr::String(str)) => {
                let str = str.replace(r"\n", "\n").replace(r"\t", "\t");
                let label = format!("literal{}", unsafe {
                    LITERAL_COUNT += 1;
                    LITERAL_COUNT
                });

                program
                    .add(Mov(Rsi, Memory::from(label.as_str()).into()))
                    .add(Mov(Rdx, (str.len() as i32).into()))
                    .add(Call("print".into()))
                    .insert_data(&label, &str)
            }
            Stmt::Print(Expr::Arexpr(arexpr)) => arexpr
                .generate(program, symbol_table)
                .add(Pop(Rsi))
                .add(Call("printn".into())),
            Stmt::NoOp => program,
        }
    }
}

impl Optimize for Stmt {
    fn optimize(self) -> Self {
        match self {
            Stmt::Print(expr) => Stmt::Print(expr.optimize()),
            Stmt::Let(ident, expr) => Stmt::Let(ident, expr.optimize()),
            Stmt::If { cond, then, els } => {
                if let Ok(b) = cond.try_execute() {
                    if b {
                        *then
                    } else {
                        els.map_or(Stmt::NoOp, |s| *s)
                    }
                } else {
                    Stmt::If { cond, then, els }
                }
            }
            _ => self,
        }
    }
}
