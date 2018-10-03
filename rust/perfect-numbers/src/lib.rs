#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = factorization(num)
        .into_iter()
        .filter(|x| *x != num) // don't include the number itself
        .sum();

    match sum {
        _ if sum == num => Some(Classification::Perfect),
        _ if sum > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient)
    }
}

pub fn factorization(num: u64) -> Vec<u64> {
    match num {
        0 => vec![],
        _ => (1..num).into_iter().filter(|x| num % x == 0).collect()
    }
}
