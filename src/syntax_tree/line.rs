use pest::iterators::Pair;

use super::Stmt;
use crate::{
    error::SyntaxError,
    generate::Generate,
    graphviz::{Node, ToNode},
    optimize::Optimize,
    parser::Rule,
    symbol_table::SymbolTable,
};

static mut LAST_LINE: usize = 0;

#[derive(Debug)]
pub struct Line {
    number: usize,
    stmt: Stmt,
}

impl Line {
    pub fn from_pair(
        value: Pair<'_, Rule>,
        symbol_table: &mut SymbolTable,
    ) -> Result<Self, SyntaxError> {
        if matches!(value.as_rule(), Rule::rem | Rule::NEWLINE) {
            return Ok(Self {
                number: 0,
                stmt: Stmt::NoOp,
            });
        }

        debug_assert_eq!(value.as_rule(), Rule::line);

        let mut number: Option<usize> = None;
        let mut stmt: Stmt = Stmt::NoOp;

        for token in value.into_inner() {
            match token.as_rule() {
                Rule::number => {
                    number = Some(token.as_str().trim().parse().unwrap());
                    if number.unwrap() <= unsafe { LAST_LINE } {
                        return Err(SyntaxError::WrongLineNumber(token.line_col().0));
                    }
                    unsafe { LAST_LINE = number.unwrap() };
                }
                Rule::stmt => stmt = Stmt::from_pair(token, symbol_table)?,
                Rule::NEWLINE => {}
                rule => unreachable!("Expected line, found {:?}", rule),
            }
        }

        Ok(Self {
            number: number.unwrap_or_else(|| {
                if matches!(stmt, Stmt::NoOp) {
                    0
                } else {
                    unsafe {
                        LAST_LINE += 1;
                        LAST_LINE
                    }
                }
            }),
            stmt,
        })
    }

    /// Checks if the line is empty.
    ///
    /// A line can be empty because of the source program or because of optimizations.
    pub fn is_empty(&self) -> bool {
        matches!(self.stmt, Stmt::NoOp)
    }
}

impl ToNode for Line {
    fn to_node(&self) -> Node {
        Node::new(&format!("line ({})", self.number)).add(self.stmt.to_node())
    }
}

impl Generate for Line {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        if matches!(self.stmt, Stmt::NoOp) {
            return program;
        }

        let program = program.label(&format!("line{}", self.number));
        self.stmt.generate(program, symbol_table)
    }
}

impl Optimize for Line {
    fn optimize(mut self) -> Self {
        self.stmt = self.stmt.optimize();
        self
    }
}
