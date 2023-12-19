use pest::iterators::Pair;

use crate::{
    graphviz::{Node, ToNode},
    optimize::Optimize,
    parser::Rule,
    symbol_table::SymbolTable,
};

mod arexpr;
pub use arexpr::*;

#[derive(Debug)]
pub enum Expr {
    String(String),
    /// Arithmetic expression
    Arexpr(Arexpr),
}

impl Expr {
    pub fn from_pair(value: Pair<'_, Rule>, symbol_table: &mut SymbolTable) -> Self {
        let inner = value.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::string => Expr::String(strip(inner.as_str())),
            Rule::arexpr => Expr::Arexpr(Arexpr::from_pair(inner, symbol_table)),
            rule => unreachable!("Expected expression, found {:?}", rule),
        }
    }
}

impl ToNode for Expr {
    fn to_node(&self) -> crate::graphviz::Node {
        match self {
            Self::String(str) => Node::new(&format!(r#""{str}""#)),
            Self::Arexpr(arexpr) => arexpr.to_node(),
        }
    }
}

impl Optimize for Expr {
    fn optimize(self) -> Self {
        match self {
            Expr::String(_) => self,
            Expr::Arexpr(arexpr) => Expr::Arexpr(arexpr.optimize()),
        }
    }
}

fn strip(str: &str) -> String {
    str.trim()
        .strip_suffix('"')
        .unwrap()
        .strip_prefix('"')
        .unwrap()
        .to_string()
}
