#![warn(clippy::pedantic)]

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    let mut primes = vec![];

    for i in 0..numbers.len() {
        let prime = match numbers[i] {
            Some(n) => n,
            None => continue,
        };

        primes.push(prime);

        for i in 2.. {
            let multiple_index = (prime * i - 2) as usize;

            match numbers.get_mut(multiple_index) {
                Some(n) => *n = None,
                None => break,
            };
        }
    }

    primes
}
