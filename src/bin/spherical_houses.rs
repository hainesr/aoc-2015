//
// Advent of Code 2015
//
// Robert Haines
//
// Public Domain
//

use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("./etc/spherical_houses.txt")
        .expect("Something went wrong reading the file.");
    let directions = input.trim();

    let visits = visit_houses(directions);
    println!("Part 1: {}", visits.len());
}

fn visit_houses(directions: &str) -> HashSet<(i32, i32)> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut visits = HashSet::new();

    // We visit the first house immediately.
    visits.insert((x, y));

    for dir in directions.chars() {
        match dir {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!("Unexpected direction in input!"),
        }

        // For each visit add to the set, which will ignore repeat visits.
        visits.insert((x, y));
    }

    visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_visits() {
        let visits = visit_houses(">");
        assert_eq!(2, visits.len());

        let visits = visit_houses("^>v<");
        assert_eq!(4, visits.len());
    }

    #[test]
    #[should_panic]
    fn bad_directions() {
        visit_houses("<>^va");
    }
}
