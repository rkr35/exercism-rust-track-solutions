use crate::graph::attributes::{Attributes, Attrs};

#[derive(PartialEq, Debug)]
pub struct Edge {
    from: String,
    to: String,
    attrs: Attributes,
}

impl Edge {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            attrs: Attributes::new(),
        }
    }

    pub fn with_attrs(mut self, attributes: Attrs) -> Self {
        self.attrs = Attributes::from(attributes);
        self
    }
}
