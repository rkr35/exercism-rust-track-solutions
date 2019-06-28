#![warn(clippy::pedantic)]

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    (0..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()?;
            numbers.iter_mut().skip(i).step_by(prime as usize).skip(1).for_each(|n| *n = None);
            Some(prime)
        })
        .collect()
}
