//! # NHS Number replacer CLI
//!
//! **[documentation](https://docs.rs/nhs-number-replacer-cli/)**
//! •
//! **[source](https://github.com/GIG-Cymru-NHS-Wales/nhs-number-replacer-cli)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/GIG-Cymru-NHS-Wales/nhs-number-replacer-cli/refs/heads/main/llms.txt)**
//! •
//! **[crate](https://crates.io/crates/nhs-number-replacer-cli)**
//! •
//! **[email](mailto:joel.henderson@wales.nhs.uk)**
//! 
//! National Health Service (NHS) Number replacer command line interface (CLI).
//!
//! ## Steps
//!
//! This program does these steps:
//!
//! 1. Read each line from standard input.
//!
//! 2. Match a regex pattern that is essentially "### ### ####".
//!
//! 3. Replace the match with a NHS Number that is a testable random sample i.e. in the NHS Number reserved range of "999 000 0000" to "999 999 9999".
//!
//! 4. Output each line to standard output.
//!
//! ## Example
//!
//! Run:
//!
//! ```sh
//! echo "111 111 1111" | nhs-number-replacer-cli
//! ```
//!
//! Output:
//!
//! ```stdout
//! 999 265 6328
//! ```
//!

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
