extern crate rand;
use rand::distributions::{IndependentSample, Range};

pub fn private_key(p: u64) -> u64 {
    Range::new(2, p).ind_sample(&mut rand::thread_rng())
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

// algorithm from wikipedia
// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn modular_pow(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result: u64 = 1;
    let mut b = b % m;
    let mut e = e;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m;
    }
    result
}
