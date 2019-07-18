#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    use std::cmp::Ordering::*;
    use Classification::*;
    match num { 0 => None, 1 => Some(Deficient), _ => {
        Some(match (2..=(num as f64).sqrt() as u64)
            .filter_map(|i| if num % i != 0 { None } 
                            else { let j = num / i; Some((i, if i == j { 0 } else { j })) })
            .fold(1, |sum, (i, j)| sum + i + j)
            .cmp(&num) { Equal => Perfect, Greater => Abundant, Less => Deficient })
    }}
}
