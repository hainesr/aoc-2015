//
// Advent of Code 2015
//
// Robert Haines
//
// Public Domain
//

use std::fs;

fn main() {
    let input = fs::read_to_string("./etc/no_math.txt")
        .expect("Something went wrong reading the file.");

    let input = input.trim();

    let parcels = parse_input(input);

    println!("Part 1: {}", part1(&parcels));
}

fn part1(parcels: &Vec<Parcel>) -> u32 {
    parcels.iter().map(|p| p.wrap_required()).sum()
}

fn parse_input(input: &str) -> Vec<Parcel> {
    input.lines().map(|l| Parcel::from_string(l)).collect()
}

struct Parcel {
    length: u32,
    width: u32,
    height: u32,
    smallest_side: u32,
}

impl Parcel {
    fn new(l: u32, w: u32, h: u32) -> Parcel {
        let sides = vec![l * w, w * h, h * l];

        Parcel {
            length: l,
            width: w,
            height: h,
            smallest_side: match sides.iter().min() {
                Some(s) => *s,
                None => 0,
            }
        }
    }

    fn from_string(s: &str) -> Parcel {
        let dims: Vec<u32> = s.split('x').map(|i| i.parse().unwrap()).collect();

        Parcel::new(dims[0], dims[1], dims[2])
    }

    fn area(&self) -> u32 {
        2 * (
            (self.length * self.width) +
            (self.width * self.height) +
            (self.height * self.length)
        )
    }

    fn wrap_required(&self) -> u32 {
        self.area() + self.smallest_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parcel_new() {
        let p = Parcel::new(2, 3, 4);

        assert_eq!(2, p.length);
        assert_eq!(3, p.width);
        assert_eq!(4, p.height);
        assert_eq!(6, p.smallest_side);
    }

    #[test]
    fn parcel_from_string() {
        let p = Parcel::from_string("2x3x4");

        assert_eq!(2, p.length);
        assert_eq!(3, p.width);
        assert_eq!(4, p.height);
        assert_eq!(6, p.smallest_side);
    }

    #[test]
    fn parcel_area_and_wrap() {
        let p = Parcel::new(2, 3, 4);

        assert_eq!(52, p.area());
        assert_eq!(58, p.wrap_required());
    }
}
