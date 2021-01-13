//
// Advent of Code 2015
//
// Robert Haines
//
// Public Domain
//

#[macro_use]
extern crate lazy_static;

use std::fs;
use fancy_regex::Regex;

const NAUGHTY: &[&str] = &["ab", "cd", "pq", "xy"];

fn main() {
    let input = fs::read_to_string("./etc/intern_elves.txt")
        .expect("Something went wrong reading the file.");
    let input = input.trim();

    println!("Part 1: {}", part1(input));
}

fn part1(strings: &str) -> u32 {
    let mut num = 0;

    for line in strings.lines() {
        if is_nice_string(line) {
            num += 1;
        }
    }

    num
}

fn is_nice_string(text: &str) -> bool {
    lazy_static! {
        static ref VOWELS: Regex = Regex::new("[aeiou].*[aeiou].*[aeiou]").unwrap();
        static ref DOUBLES: Regex = Regex::new("(.)\\1{1,}").unwrap();
    }

    for n in NAUGHTY {
        if text.contains(n) {
            return false;
        }
    }

    match VOWELS.is_match(text) {
        Ok(v) => if !v { return false },
        Err(_) => return false,
    }

    match DOUBLES.is_match(text) {
        Ok(v) => if !v { return false },
        Err(_) => return false,
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_strings() {
        assert!(is_nice_string("ugknbfddgicrmopn"));
        assert!(is_nice_string("aaa"));
        assert!(is_nice_string("haegwjzuvuyypxyu") == false); // Contains 'xy'.
        assert!(is_nice_string("dvszwmarrgswjxmb") == false); // Only one vowel.
        assert!(is_nice_string("jchzalrnumimnmhp") == false); // No double letter.
    }
}
