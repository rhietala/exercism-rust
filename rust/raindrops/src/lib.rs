// https://doc.rust-lang.org/1.9.0/book/strings.html

use itertools::Itertools;

pub fn fizzbuzzer(n: u32, divisor: u32, s: String) -> Option<String> {
    if n % divisor == 0 {
        Some(s)
    } else {
        None
    }
}

pub fn raindrops(n: u32) -> String {
    let drops = [fizzbuzzer(n, 3, "Pling".to_string()),
                 fizzbuzzer(n, 5, "Plang".to_string()),
                 fizzbuzzer(n, 7, "Plong".to_string())];
    match drops {
        [None, None, None] => n.to_string(),
        _ => drops
            .iter()
            .map(|x| x.unwrap_or("".to_string()))
            .join("")
    }
}
