use std::{
    error::Error,
    fs::{self, File},
    io::prelude::Write,
    path::PathBuf,
    process::Command,
};

use cli::Cli;
use generate::{generate, OPTIMIZED, PARSE_TREE_DOT_FILE, UNOPTIMIZED};
use graphviz::{compile_dot, Digraph, ToNodes};
use optimize::Optimize;
use parser::renamed_rules;
use pest::Parser;
use symbol_table::SymbolTable;
use syntax_tree::SyntaxTree;

use crate::parser::TbParser;

mod cli;
mod error;
mod generate;
mod graphviz;
mod optimize;
mod parser;
mod symbol_table;
mod syntax_tree;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    let mut symbol_table = SymbolTable::default();

    let ast = parse(&cli.file, &mut symbol_table)?;

    generate(&ast, &mut symbol_table, &UNOPTIMIZED, false)?;

    // optimize AST
    let ost = ast.optimize();
    generate(&ost, &mut symbol_table, &OPTIMIZED, true)?;

    if cli.run {
        match Command::new(format!("./{}", OPTIMIZED.bin)).status() {
            Ok(status) => println!("{status}"),
            Err(e) => eprintln!("{e}"),
        };
    }

    Ok(())
}

fn parse(file: &PathBuf, symbol_table: &mut SymbolTable) -> Result<SyntaxTree, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    // parse file
    let parsed = TbParser::parse(parser::Rule::file, &content);

    let Ok(parsed) = parsed else {
        let err = parsed.unwrap_err();
        eprintln!(
            "{}",
            err.renamed_rules(renamed_rules)
                .with_path(file.to_str().unwrap())
        );

        return Err("syntax error".into());
    };

    // Write parse tree
    {
        let mut file = File::create(PARSE_TREE_DOT_FILE)?;
        file.write_all(
            Digraph::new(parsed.to_nodes().first().unwrap().clone())
                .to_string()
                .as_bytes(),
        )?;
    }
    if let Err(e) = compile_dot(PARSE_TREE_DOT_FILE) {
        eprintln!("{e}");
        eprintln!("Graphviz might not be installed. See https://graphviz.org/download/");
    };

    // convert to syntax tree
    let ast = SyntaxTree::from_pairs(parsed, symbol_table);
    let Ok(ast) = ast else {
        let err = ast.unwrap_err();
        eprintln!("{err}");
        return Err(Box::new(err));
    };

    Ok(ast)
}
