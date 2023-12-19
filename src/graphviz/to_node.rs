use crate::graphviz::Node;

/// Converts something to one graphviz node
pub trait ToNode {
    fn to_node(&self) -> Node;
}

/// Converts something to multiple graphviz nodes
pub trait ToNodes {
    fn to_nodes(&self) -> Vec<Node>;
}

impl ToNode for String {
    fn to_node(&self) -> Node {
        Node::new(self)
    }
}

/// Implements [`ToNode`] for types that can be converted to string
macro_rules! impl_to_node_to_string {
    ($ty:ty) => {
        impl ToNode for $ty {
            fn to_node(&self) -> Node {
                Node::new(&self.to_string())
            }
        }
    };
}

impl_to_node_to_string!(i8);
impl_to_node_to_string!(i16);
impl_to_node_to_string!(i32);
impl_to_node_to_string!(i64);
impl_to_node_to_string!(u8);
impl_to_node_to_string!(u16);
impl_to_node_to_string!(u32);
impl_to_node_to_string!(u64);
impl_to_node_to_string!(f32);
impl_to_node_to_string!(f64);
