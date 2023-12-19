//! Contains everything graphviz related.

mod digraph;
mod node;
mod to_node;

use std::{
    io,
    process::{Command, ExitStatus},
};

pub use digraph::*;
pub use node::*;
pub use to_node::*;

/// Compiles the given graphviz dot file.
///
/// Returns whether the underlying command successfully ran.
pub fn compile_dot(filename: &str) -> io::Result<ExitStatus> {
    Command::new("dot")
        .arg("-Tsvg")
        .arg("-O")
        .arg(filename)
        .status()
}
