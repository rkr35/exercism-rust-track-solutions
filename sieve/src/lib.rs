#![warn(clippy::pedantic)]

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    (0..numbers.len())
        .filter_map(|i| numbers[i].map(|prime| {
            (prime..=upper_bound).step_by(prime as usize).skip(1).for_each(|i| numbers[(i - 2) as usize] = None); 
            prime
        }))
        .collect()
}
