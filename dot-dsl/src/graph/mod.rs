pub mod graph_items;
use graph_items::edge::Edge;
use graph_items::node::Node;

mod attributes;
use attributes::{Attributes, Attrs};

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: Attributes,
}

impl Graph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_edges(mut self, edges: Vec<Edge>) -> Self {
        self.edges = edges;
        self
    }

    pub fn with_attrs(mut self, attributes: Attrs) -> Self {
        self.attrs = Attributes::from(attributes);
        self
    }

    pub fn get_node(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.label == label)
    }
}
