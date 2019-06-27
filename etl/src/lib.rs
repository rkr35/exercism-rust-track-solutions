use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new = BTreeMap::new();

    for (&points, characters) in h.iter() {
        for c in characters {
            new.insert(c.to_lowercase().next().unwrap(), points);
        }        
    }

    new
}
