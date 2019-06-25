fn verse(i: usize, list: &[&str]) -> String {
    match i {
        n if n == list.len() - 1 => format!("And all for the want of a {}.", list[0]),
        _ => format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]),
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    (0..list.len())
        .map(|i| verse(i, list))
        .collect()
}
