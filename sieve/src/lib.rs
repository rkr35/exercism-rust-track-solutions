#![warn(clippy::pedantic)]

type MarkableNumber = Option<u64>;

fn mark_multiples(numbers: &mut [MarkableNumber], prime: u64) -> u64 {
    let length = numbers.len();

    (2..)
        .map(|i| (prime * i - 2) as usize)
        .take_while(|&i| i < length)
        .for_each(|i| numbers[i] = None );

    prime
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(MarkableNumber::from).collect();

    (0..numbers.len())
        .filter_map(|i| numbers[i].map(|prime| mark_multiples(&mut numbers, prime)))
        .collect()
}
