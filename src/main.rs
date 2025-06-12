use std::io::{self, BufRead};
use std::sync::LazyLock;
use regex::{Regex, Captures};
use std::borrow::Cow;

static REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"\b(\d\d\d \d\d\d \d\d\d\d)\b").unwrap());

pub fn replacer(_captures: &Captures) -> String {
    nhs_number::testable_random_sample().into()
}

pub fn cook(s: &str) -> Cow<'_, str> {
    REGEX.replace_all(s, replacer)
}

fn main() {
    let stdin  = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => {
                println!("{}", cook(&line));
            }
            Err(e) => {
                eprintln!("Error reading line {}. Error: {}", i, e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replacer() {
        let captures = REGEX.captures("123 456 7890").unwrap();
        let actual = replacer(&captures);
        assert_eq!(actual.len(), 12);
        assert!(actual.starts_with("999 "));
    }


    #[test]
    fn test_cook() {
        let haystack = "123 456 7890";
        let actual = cook(&haystack);
        assert_eq!(actual.len(), 12);
        assert!(actual.starts_with("999 "));
    }

}