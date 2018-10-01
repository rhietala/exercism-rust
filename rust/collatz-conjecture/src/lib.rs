pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps: u64 = 0;
    loop {
        match n {
            0 => return None, // negative values are not possible because n is u64
            1 => return Some(steps),
            _ if n % 2 == 0 => n /= 2,
            _ => n = n * 3 + 1
        }
        steps += 1;
    }
}
