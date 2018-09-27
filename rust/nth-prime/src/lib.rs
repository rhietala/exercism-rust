pub fn nth(n: u32) -> u32 {
    // collect all primes to vector
    let mut primes: Vec<u32> = vec![2];

    // x is the current prime candidate
    let mut x = 3;

    // loop until nth prime is found
    while primes.len() <= n as usize {
        let x_is_prime: bool = primes
            .clone()
            .into_iter()
            .map(|prime| x % prime == 0) // for all primes found, check if x is divisible by prime
            .filter(|b| *b) // only care if it is divisible
            .collect::<Vec<bool>>()
            .len() == 0; // x is prime if it is not divisible by any primes up until x

        if x_is_prime {
            primes.push(x);
        }

        x = x + 1;
    }

    return primes[n as usize];
}
