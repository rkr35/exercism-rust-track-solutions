pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut odd_primes = vec![3];

    while odd_primes.len() != n as usize {
        let first_number_to_test = odd_primes.last().unwrap() + 2;

        let next_prime = (first_number_to_test..)
            .step_by(2)
            .find(|i| odd_primes.iter().all(|p| i % p != 0));

        odd_primes.push(next_prime.unwrap());
    }

    odd_primes.pop().unwrap()
}
