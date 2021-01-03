//
// Advent of Code 2015
//
// Robert Haines
//
// Public Domain
//

use std::fs;

fn main() {
    let input = fs::read_to_string("./etc/not_quite_lisp.txt")
        .expect("Something went wrong reading the file.");

    let input = input.trim();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut up = 0;

    for c in input.chars() {
        if c == '(' {
            up += 1
        }
    }

    up - ((input.len() as i32) - up)
}

fn part2(input: &str) -> i32 {
    let mut floor = 0;
    let mut result = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1
        } else {
            floor -= 1
        }

        if floor == -1 {
            result = (i as i32) + 1;
            break;
        }
    }

    result
}
