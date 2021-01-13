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
    println!("Part 2: {}", part2(input));
}

fn part1(strings: &str) -> u32 {
    let mut num = 0;

    for line in strings.lines() {
        if is_nice_string1(line) {
            num += 1;
        }
    }

    num
}

fn part2(strings: &str) -> u32 {
    let mut num = 0;

    for line in strings.lines() {
        if is_nice_string2(line) {
            num += 1;
        }
    }

    num
}

fn is_nice_string1(text: &str) -> bool {
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

fn is_nice_string2(text: &str) -> bool {
    lazy_static! {
        static ref DOUBLE: Regex = Regex::new("(..).*\\1").unwrap();
        static ref BETWEEN: Regex = Regex::new("(.).\\1").unwrap();
    }

    match BETWEEN.is_match(text) {
        Ok(v) => if !v { return false },
        Err(_) => return false,
    }

    match DOUBLE.is_match(text) {
        Ok(v) => if !v { return false },
        Err(_) => return false,
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_strings1() {
        assert!(is_nice_string1("ugknbfddgicrmopn"));
        assert!(is_nice_string1("aaa"));
        assert!(is_nice_string1("haegwjzuvuyypxyu") == false); // Contains 'xy'.
        assert!(is_nice_string1("dvszwmarrgswjxmb") == false); // Only one vowel.
        assert!(is_nice_string1("jchzalrnumimnmhp") == false); // No double letter.
    }

    #[test]
    fn nice_strings2() {
        assert!(is_nice_string2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_string2("xxyxx"));
        assert!(is_nice_string2("uurcxstgmygtbstg") == false); // No repeat with single letter.
        assert!(is_nice_string2("ieodomkazucvgmuy") == false); // No pair appearing twice.
    }
}
