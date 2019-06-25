use std::collections::HashMap;

pub type Attrs<'a> = &'a [(&'a str, &'a str)];

type AttributeMap = HashMap<String, String>;

#[derive(Default, PartialEq, Debug)]
pub struct Attributes {
    attributes: AttributeMap,
}

impl Attributes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, attribute_key: &str) -> Option<&str> {
        self.attributes.get(attribute_key).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

impl<'a> From<Attrs<'a>> for Attributes {
    fn from(attributes: Attrs) -> Self {
        Self {
            attributes: attributes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

// Attributes == AttributeMap
impl PartialEq<AttributeMap> for Attributes {
    fn eq(&self, other: &AttributeMap) -> bool {
        self.attributes == *other
    }
}
