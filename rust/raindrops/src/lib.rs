// https://doc.rust-lang.org/1.9.0/book/strings.html

extern crate itertools;
use itertools::Itertools;

pub fn fizzbuzzer(n: u32, divisor: u32, s: &str) -> Option<String> {
    if n % divisor == 0 {
        Some(String::from(s))
    } else {
        None
    }
}

pub fn raindrops(n: u32) -> String {
    match (fizzbuzzer(n, 3, "Pling"),
           fizzbuzzer(n, 5, "Plang"),
           fizzbuzzer(n, 7, "Plong")) {
        (None, None, None) => n.to_string(),
        (a, b, c) => [a, b, c]
            .iter()
            .map(|x| x.clone().unwrap_or(String::from("")))
            .join("")
    }
}
