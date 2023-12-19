mod renamed_rules;

use pest::iterators::{Pair, Pairs};
pub use renamed_rules::renamed_rules;

use crate::graphviz::{Node, ToNode, ToNodes};

/// TinyBASIC parser
#[derive(pest_derive::Parser)]
#[grammar = "parser/tinybasic.pest"]
pub struct TbParser;

impl ToNodes for Pairs<'_, Rule> {
    fn to_nodes(&self) -> Vec<Node> {
        let pairs = self.clone();

        pairs.map(|pair| pair.to_node()).collect()
    }
}

impl ToNode for Pair<'_, Rule> {
    fn to_node(&self) -> Node {
        let name = format!("{:?}", self.as_rule());
        let mut node = Node::new(&name);

        if matches!(self.as_rule(), Rule::string | Rule::number | Rule::ident) {
            node = node.add(Node::new(self.as_str()));
        }

        for n in self.clone().into_inner().to_nodes() {
            node = node.add(n);
        }

        node
    }
}
