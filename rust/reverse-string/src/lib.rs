pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input
        .chars()
        .collect();

    // have to do this on its own line, otherwise it complains that
    // found type is ()
    // maybe something to do with .reverse() mutating the value in place
    chars.reverse();

    return chars.into_iter().collect();
}
