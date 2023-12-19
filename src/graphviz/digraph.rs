use std::fmt::Display;

use super::Node;

/// A graphviz directed graph
pub struct Digraph {
    root: Node,
}

impl Digraph {
    /// Creates a new directed graph from a root node.
    pub fn new(root: Node) -> Self {
        Self { root }
    }
}

impl Display for Digraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "digraph {{\n{}}}", self.root)
    }
}
