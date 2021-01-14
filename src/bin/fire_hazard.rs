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

fn main() {
    let input = fs::read_to_string("./etc/fire_hazard.txt")
        .expect("Something went wrong reading the file.");
    let input = input.trim();

    println!("Part 1: {}", part1(input));
}

fn part1(commands: &str) -> u32 {
    let mut lights = LightingGrid::new();

    for command in commands.lines() {
        lights.run_command(command);
    }

    lights.brightness()
}

pub struct LightingGrid {
    grid: [[u8; LightingGrid::SIDE_LENGTH]; LightingGrid::SIDE_LENGTH],
}

impl LightingGrid {
    const SIDE_LENGTH: usize = 1000;
    const CMD_REGEX: &'static str = r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$";

    pub fn new() -> LightingGrid {
        LightingGrid {
            grid: [[0; LightingGrid::SIDE_LENGTH]; LightingGrid::SIDE_LENGTH],
        }
    }

    pub fn run_command(&mut self, cmd: &str) {
        let (keyword, lx, ly, hx, hy) = LightingGrid::parse_command(cmd);

        match keyword {
            "turn on" => self.turn_on(lx, ly, hx, hy),
            "turn off" => self.turn_off(lx, ly, hx, hy),
            "toggle" => self.toggle(lx, ly, hx, hy),
            &_ => panic!("Bad command"),
        }
    }

    pub fn turn_on(&mut self, lx: usize, ly: usize, hx: usize, hy: usize) {
        for x in lx..=hx {
            for y in ly..=hy {
                self.grid[x][y] = 1;
            }
        }
    }

    pub fn turn_off(&mut self, lx: usize, ly: usize, hx: usize, hy: usize) {
        for x in lx..=hx {
            for y in ly..=hy {
                self.grid[x][y] = 0;
            }
        }
    }

    pub fn toggle(&mut self, lx: usize, ly: usize, hx: usize, hy: usize) {
        for x in lx..=hx {
            for y in ly..=hy {
                self.grid[x][y] = 1 - self.grid[x][y];
            }
        }
    }

    pub fn brightness(&self) -> u32 {
        self.grid.iter().flat_map(|r| r.iter().map(|&c| c as u32)).sum()
    }

    fn parse_command(cmd: &str) -> (&str, usize, usize, usize, usize) {
        lazy_static! {
            static ref CMD: Regex = Regex::new(LightingGrid::CMD_REGEX).unwrap();
        }

        let tokens = CMD.captures(cmd).expect("Regex error.").expect("No match.");

        let keyword = tokens.get(1).expect("Group missing.").as_str();
        let lx = tokens.get(2).expect("Group missing.").as_str().parse::<usize>().expect("Error");
        let ly = tokens.get(3).expect("Group missing.").as_str().parse::<usize>().expect("Error");
        let hx = tokens.get(4).expect("Group missing.").as_str().parse::<usize>().expect("Error");
        let hy = tokens.get(5).expect("Group missing.").as_str().parse::<usize>().expect("Error");

        (keyword, lx, ly, hx, hy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid() {
        let lights = LightingGrid::new();
        assert_eq!(0, lights.grid[0][0]);
        assert_eq!(0, lights.grid[999][999]);
        assert_eq!(0, lights.brightness());
    }

    #[test]
    fn lights_on_off() {
        let mut lights = LightingGrid::new();

        lights.turn_on(0, 0, 999, 999);
        assert_eq!(1, lights.grid[0][0]);
        assert_eq!(1, lights.grid[999][999]);
        assert_eq!(1_000_000, lights.brightness());

        lights.turn_off(500, 0, 999, 999);
        assert_eq!(1, lights.grid[0][0]);
        assert_eq!(1, lights.grid[499][0]);
        assert_eq!(0, lights.grid[500][0]);
        assert_eq!(0, lights.grid[999][999]);
        assert_eq!(500_000, lights.brightness());
    }

    #[test]
    fn lights_toggle() {
        let mut lights = LightingGrid::new();

        lights.toggle(500, 0, 999, 999);
        assert_eq!(0, lights.grid[0][0]);
        assert_eq!(0, lights.grid[499][0]);
        assert_eq!(1, lights.grid[500][0]);
        assert_eq!(1, lights.grid[999][999]);

        lights.toggle(0, 0, 999, 999);
        assert_eq!(1, lights.grid[0][0]);
        assert_eq!(1, lights.grid[499][0]);
        assert_eq!(0, lights.grid[500][0]);
        assert_eq!(0, lights.grid[999][999]);
    }

    #[test]
    fn parse_cmd() {
        let (cmd, lx, ly, hx, hy) = LightingGrid::parse_command("toggle 0,0 through 999,999");
        assert_eq!("toggle", cmd);
        assert_eq!(0, lx);
        assert_eq!(0, ly);
        assert_eq!(999, hx);
        assert_eq!(999, hy);

        let (cmd, lx, ly, hx, hy) = LightingGrid::parse_command("turn on 5,10 through 600,500");
        assert_eq!("turn on", cmd);
        assert_eq!(5, lx);
        assert_eq!(10, ly);
        assert_eq!(600, hx);
        assert_eq!(500, hy);

        let (cmd, lx, ly, hx, hy) = LightingGrid::parse_command("turn off 20,500 through 21,501");
        assert_eq!("turn off", cmd);
        assert_eq!(20, lx);
        assert_eq!(500, ly);
        assert_eq!(21, hx);
        assert_eq!(501, hy);
    }

    #[test]
    fn run() {
        let mut lights = LightingGrid::new();
        assert_eq!(0, lights.brightness());

        lights.run_command("turn on 0,0 through 999,999");
        assert_eq!(1_000_000, lights.brightness());

        lights.run_command("toggle 0,0 through 999,0");
        assert_eq!(999_000, lights.brightness());

        lights.run_command("turn off 499,499 through 500,500");
        assert_eq!(998_996, lights.brightness());
    }
}
