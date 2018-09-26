extern crate itertools;
use itertools::Itertools;

pub fn reverse(input: &str) -> String {
    let reversed: Vec<char> = input
        .chars()
        .reverse();

    reversed
}
