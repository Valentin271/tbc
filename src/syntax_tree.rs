use pest::iterators::Pairs;

use self::line::Line;
use crate::{
    error::SyntaxError,
    generate::Generate,
    graphviz::{Node, ToNode},
    optimize::Optimize,
    parser::Rule,
    symbol_table::SymbolTable,
};

mod cond;
mod expr;
mod line;
mod stmt;

pub use cond::*;
pub use expr::*;
pub use line::*;
pub use stmt::*;

#[derive(Debug)]
pub struct SyntaxTree {
    lines: Vec<Line>,
}

impl SyntaxTree {
    pub fn from_pairs(
        mut value: Pairs<'_, Rule>,
        symbol_table: &mut SymbolTable,
    ) -> Result<Self, SyntaxError> {
        let mut lines = vec![];

        let file = value.next().expect("Empty parse tree");

        debug_assert_eq!(file.as_rule(), Rule::file);

        for line in file.into_inner() {
            if line.as_rule() == Rule::EOI {
                break;
            }
            lines.push(Line::from_pair(line, symbol_table)?)
        }

        Ok(Self { lines })
    }
}

impl ToNode for SyntaxTree {
    fn to_node(&self) -> Node {
        let mut node = Node::new("program");
        for line in &self.lines {
            if !line.is_empty() {
                node = node.add(line.to_node());
            }
        }
        node
    }
}

impl Generate for SyntaxTree {
    fn generate(
        &self,
        program: tiny_elf::asm::Program,
        symbol_table: &mut SymbolTable,
    ) -> tiny_elf::asm::Program {
        use tiny_elf::asm::{Mnemonic::*, Register::*};

        // R15 is used as the stack base pointer
        // allocate stack space
        let mut program = program
            .add(Mov(R15, Rsp.into()))
            .add(Sub(Rsp, (symbol_table.size() as i32).into()));

        for line in &self.lines {
            program = line.generate(program, symbol_table);
        }

        // don't execute function
        let program = program.add(Jmp("exit".into()));

        let program = program
            .func("printn")
            // init
            .add(Mov(Rax, Rsi.into()))
            .add(Xor(Rcx, Rcx.into()))
            .add(Mov(Rbx, 10.into()))
            // loop over digit and store on stack
            .add(Jmp("printn_inner_cond".into()))
            .label("printn_inner")
            .add(Xor(Rdx, Rdx.into()))
            .add(IDiv(Rbx))
            .add(Add(Rdx, ('0' as i32).into()))
            .add(Push(Rdx.into()))
            .add(Inc(Rcx))
            .label("printn_inner_cond")
            .add(Cmp(Rax, 10.into()))
            .add(Jge("printn_inner".into()))
            // handle last digit
            .add(Add(Rax, ('0' as i32).into()))
            .add(Push(Rax.into()))
            .add(Inc(Rcx))
            // setup print params
            .add(Mov(Rsi, Rsp.into()))
            .add(IMul(Rcx, 8.into()))
            .add(Mov(Rdx, Rcx.into()))
            .add(Call("print".into()))
            .func_end();

        let program = program
            .func("print")
            .add(Mov(Rax, 1.into()))
            .add(Mov(Rdi, 1.into()))
            .add(Syscall)
            .func_end();

        let program = program
            .func("read")
            .add(Mov(Rax, 0.into()))
            .add(Mov(Rdi, 0.into()))
            .add(Syscall)
            .func_end();

        program
            .label("exit")
            .add(Mov(Rax, 60.into()))
            .add(Mov(Rdi, 0.into()))
            .add(Syscall)
    }
}

impl Optimize for SyntaxTree {
    fn optimize(mut self) -> Self {
        let mut lines = Vec::new();

        for line in self.lines {
            let line = line.optimize();
            if !line.is_empty() {
                lines.push(line);
            }
        }

        self.lines = lines;

        self
    }
}
