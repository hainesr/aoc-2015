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

    let (evens, odds) = divide_directions(directions);
    let santa = visit_houses(evens.as_str());
    let robo = visit_houses(odds.as_str());
    let visits = santa.union(&robo).collect::<HashSet<_>>();
    println!("Part 2: {}", visits.len());
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

fn divide_directions(directions: &str) -> (String, String) {
    let mut s1 = String::new();
    let mut s2 = String::new();

    for (i, d) in directions.char_indices() {
        if i % 2 == 0 {
            s1.push(d);
        } else {
            s2.push(d);
        }
    }

    (s1, s2)
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

    #[test]
    fn split_dirs() {
        let (one, two) = divide_directions("^v^v^v^v");
        assert_eq!("^^^^", one);
        assert_eq!("vvvv", two);
    }
}
