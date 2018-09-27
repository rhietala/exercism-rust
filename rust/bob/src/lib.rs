extern crate regex;
use regex::Regex;

pub fn reply(message: &str) -> &str {
    match message {
        _ if shouting(message) && asking(message) => "Calm down, I know what I'm doing!",
        _ if shouting(message) => "Whoa, chill out!",
        _ if asking(message) => "Sure.",
        _ if silence(message) => "Fine. Be that way!",
        _ => "Whatever."
    }
}

// shout must have an upcase letter, and no downcase letters
pub fn shouting(message: &str) -> bool {
    let upcase_letter = Regex::new(r"[A-Z]").unwrap();
    let downcase_letter = Regex::new(r"[a-z]").unwrap();
    return upcase_letter.is_match(message) && !downcase_letter.is_match(message);
}

// question must end with a question mark that might be followed by whitespace
pub fn asking(message: &str) -> bool {
    let re = Regex::new(r"\?\s*$").unwrap();
    return re.is_match(message);
}

// silence is only whitespace
pub fn silence(message: &str) -> bool {
    let re = Regex::new(r"^\s*$").unwrap();
    return re.is_match(message);
}
