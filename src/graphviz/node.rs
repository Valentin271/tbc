use std::fmt::Display;

static mut NODE_COUNT: u32 = 0;

// A basic graphviz node
#[derive(Clone)]
pub struct Node {
    label: String,
    children: Vec<Node>,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.escape_default().to_string(),
            children: Vec::new(),
        }
    }

    /// Adds children to this node with a fluent pattern
    pub fn add(mut self, node: Node) -> Self {
        self.children.push(node);
        self
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("node{}", unsafe {
            NODE_COUNT += 1;
            NODE_COUNT
        });
        let mut node = format!("{} [label=\"{}\"];\n", name, self.label);

        for (_, child) in self.children.iter().enumerate() {
            // render link with child
            let child_name = format!("node{}", unsafe { NODE_COUNT + 1 });
            node += &format!("{} -> {};\n", name, child_name);

            // render child
            node += &child.to_string();
        }

        write!(f, "{}", node)
    }
}
