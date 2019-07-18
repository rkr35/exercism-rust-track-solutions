#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    use std::cmp::Ordering::*;
    use Classification::*;

    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Deficient);
    }

    let upper_bound = (num as f64).sqrt() as u64;
    let mut sum = 1;
    for i in 2..=upper_bound {
        if num % i == 0 {
            sum += i;
            let other_factor = num / i;

            if other_factor != i {
                sum += other_factor;
            }
        }
    }

    Some(match sum.cmp(&num) {
        Equal => Perfect,
        Greater => Abundant,
        Less => Deficient
    })
}
