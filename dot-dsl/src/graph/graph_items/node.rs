use crate::graph::attributes::{Attributes, Attrs};

#[derive(PartialEq, Debug)]
pub struct Node {
    pub label: String,
    pub attrs: Attributes,
}

impl Node {
    pub fn new(label: String) -> Self {
        Self {
            label,
            attrs: Attributes::new(),
        }
    }

    pub fn with_attrs(mut self, attributes: Attrs) -> Self {
        self.attrs = Attributes::from(attributes);
        self
    }

    pub fn get_attr(&self, attribute: &str) -> Option<&str> {
        self.attrs.get(attribute)
    }
}
