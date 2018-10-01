pub fn factors(n: u64) -> Vec<u64> {
    let primes = primes_until((n as f64).sqrt().ceil() as u64);
    let mut factors: Vec<u64> = vec![];
    let mut x = n;

    loop {
        match find_smallest_factor(x, &primes) {
            Some(y) => {
                factors.push(y);
                x = x / y;
            },
            None if x != 1 => {
                factors.push(x);
                break;
            },
            None => {
                break;
            }
        }
    }

    return factors;
}

pub fn find_smallest_factor(x: u64, primes: &Vec<u64>) -> Option<u64> {
    for prime in primes.clone().into_iter() {
        if x % prime == 0 {
            return Some(prime);
        }
        // optimization: we have to check primes only up until sqrt(x)
        if prime > (x as f64).sqrt().ceil() as u64 {
            break;
        }
    }

    return None;
}

// this function is reused from the exercise nth prime

pub fn primes_until(n: u64) -> Vec<u64> {
    // collect all primes to vector
    let mut primes: Vec<u64> = vec![2];

    // x is the current prime candidate
    let mut x: u64 = 3;

    // loop until nth prime is found
    while x <= n {
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

    return primes;
}
