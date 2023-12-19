use std::{fs::File, io::Write, process::Command};

use tiny_elf::{
    asm::{AsAsm, Program},
    bytes::AsBytes,
    program_header::Flags,
    Elf,
};

use crate::{
    graphviz::{compile_dot, Digraph, ToNode},
    optimize::Optimize,
    symbol_table::SymbolTable,
    syntax_tree::SyntaxTree,
};

pub const PARSE_TREE_DOT_FILE: &str = "parse_tree.dot";

pub const OPTIMIZED: Names = Names {
    ast: "ost.dot",
    asm: "dump.asm",
    bin: "dump.elf",
};

pub const UNOPTIMIZED: Names = Names {
    ast: "ast.dot",
    asm: "udump.asm",
    bin: "udump.elf",
};

/// This trait represents elements that can be converted to assembly for code generation
pub trait Generate {
    fn generate(&self, program: Program, symbol_table: &mut SymbolTable) -> Program;
}

pub struct Names<'a> {
    pub ast: &'a str,
    pub asm: &'a str,
    pub bin: &'a str,
}

pub fn generate(
    ast: &SyntaxTree,
    symbol_table: &mut SymbolTable,
    names: &Names,
    optimize: bool,
) -> std::io::Result<()> {
    // Write ast
    {
        let mut file = File::create(names.ast)?;
        file.write_all(Digraph::new(ast.to_node()).to_string().as_bytes())?;
    }

    // generate assembly program
    let mut program = ast.generate(Program::default(), symbol_table);
    if optimize {
        program = program.optimize();
    }
    {
        let mut file = File::create(names.asm)?;
        file.write_all(program.as_asm().as_bytes())?;
    }

    // generate binary from assembly
    let mut elf = Elf::new(program.clone());
    elf.add_data(program.data(), Flags::all());
    elf.backpatch();
    {
        let mut file = File::create(names.bin)?;
        file.write_all(&elf.as_bytes())?;
    }

    if let Err(e) = compile_dot(names.ast) {
        eprintln!("{e}");
        eprintln!("Graphviz might not be installed. See https://graphviz.org/download/");
    };

    Command::new("chmod").arg("u+x").arg(names.bin).status()?;

    Ok(())
}
