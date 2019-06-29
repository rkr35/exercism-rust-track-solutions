use std::cmp::Ordering;

pub fn find<Element, Container>(array: Container, key: Element) -> Option<usize>
where
    Element: Ord,
    Container: AsRef<[Element]>,
{
    let array = array.as_ref();
    let (mut low, mut high) = (0, array.len().checked_sub(1)?);

    while low <= high {
        let middle = low + (high - low) / 2;

        match key.cmp(array.get(middle)?) {
            Ordering::Less => high = middle.checked_sub(1)?,
            Ordering::Greater => low = middle + 1,
            Ordering::Equal => return Some(middle),
        };
    }

    None
}
