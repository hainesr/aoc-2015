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
use std::collections::HashMap;
use fancy_regex::Regex;

const INST_REGEX: &str = r"^(.+) -> ([a-z]+)$";

fn main() {
    let input = fs::read_to_string("./etc/assembly_required.txt")
        .expect("Something went wrong reading the file.");
    let input = input.trim();
    let instructions = parse_instructions(input);

    let a = get_signal("a", &instructions);
    println!("Part 1: {}", a);
}

fn get_signal(wire: &str, instructions: &HashMap<&str, Gate>) -> u16 {
    fn run_instructions<'s>(wire: &'s str, insts: &'s HashMap<&str, Gate>,
                        memo: &mut HashMap<&'s str, u16>) -> u16 {

        if memo.contains_key(wire) {
            return *memo.get(wire).unwrap();
        }

        match insts.get(wire) {
            None => panic!("No wire {}!", wire),
            Some(gate) => {
                match gate {
                    Gate::None(p) => match p {
                        Parameter::Label(l) => {
                            let r = run_instructions(l, insts, memo);
                            memo.insert(l, r);
                            r
                        },
                        Parameter::Value(v) => {
                            memo.insert(wire, *v);
                            *v
                        },
                    },

                    Gate::Not(np) => {
                        let r = run_instructions(np, insts, memo);
                        memo.insert(np, r);
                        !r
                    },

                    Gate::Or(o1, o2) => {
                        let r = run_instructions(o1, insts, memo);
                        let s = run_instructions(o2, insts, memo);
                        memo.insert(o1, r);
                        memo.insert(o2, s);
                        r | s
                    },

                    Gate::And(a1, a2) => match a1 {
                        Parameter::Label(la) => {
                            let r = run_instructions(la, insts, memo);
                            let s = run_instructions(a2, insts, memo);
                            memo.insert(la, r);
                            memo.insert(a2, s);
                            r & s
                        },
                        Parameter::Value(va) => {
                            let r = run_instructions(a2, insts, memo);
                            memo.insert(a2, r);
                            *va & r
                        }
                    },

                    Gate::Lshift(l1, l2) => {
                        let r = run_instructions(l1, insts, memo);
                        memo.insert(l1, r);
                        r << l2
                    },

                    Gate::Rshift(r1, r2) => {
                        let r = run_instructions(r1, insts, memo);
                        memo.insert(r1, r);
                        r >> r2
                    },
                }
            },
        }
    }

    let mut cache: HashMap<&str, u16> = HashMap::new();
    run_instructions(wire, instructions, &mut cache)
}

fn parse_instructions(input: &str) -> HashMap<&str, Gate> {
    let mut instructions = HashMap::new();

    for line in input.lines() {
        let (key, value) = parse_line(line);
        instructions.insert(key, value);
    }

    instructions
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Parameter<'a> {
    Label(&'a str),
    Value(u16),
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Gate<'b> {
    None(Parameter<'b>),
    Not(&'b str),
    And(Parameter<'b>, &'b str),
    Or(&'b str, &'b str),
    Lshift(&'b str, u8),
    Rshift(&'b str, u8),
}

fn parse_line<'c>(line: &'c str) -> (&'c str, Gate<'c>) {
    lazy_static! {
        static ref INST: Regex = Regex::new(INST_REGEX).unwrap();
    }

    let tokens = INST.captures(line).expect("Regex error.").expect("No match.");

    let lhs = tokens.get(1).expect("LHS missing.").as_str();
    let sink = tokens.get(2).expect("Sink missing.").as_str();

    (sink, parse_gate(lhs))
}

fn parse_gate(gate: &str) -> Gate {
    let tokens: Vec<&str> = gate.split(' ').collect();

    match tokens.len() {
        1 => {
            match tokens[0].parse::<u16>() {
                Ok(n) => Gate::None(Parameter::Value(n)),
                Err(_) => Gate::None(Parameter::Label(tokens[0])),
            }
        },
        2 => {
            Gate::Not(tokens[1])
        },
        3 => {
            match tokens[1] {
                "AND" => parse_and(tokens[0], tokens[2]),
                "OR" => Gate::Or(tokens[0], tokens[2]),
                "LSHIFT" => {
                    let shift = tokens[2].parse::<u8>().unwrap();
                    Gate::Lshift(tokens[0], shift)
                },
                "RSHIFT" => {
                    let shift = tokens[2].parse::<u8>().unwrap();
                    Gate::Rshift(tokens[0], shift)
                },
                _ => panic!("Bad op!"),
            }
        },
        _ => panic!("Bad gate!"),
    }
}

fn parse_and<'d>(arg1: &'d str, arg2: &'d str) -> Gate<'d> {
    let arg = match arg1.parse::<u16>() {
        Ok(n) => Parameter::Value(n),
        Err(_) => Parameter::Label(arg1),
    };

    Gate::And(arg, arg2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_gate() {
        assert_eq!(
            Gate::And(Parameter::Label("aa"), "bb"), parse_and("aa", "bb")
        );
        assert_eq!(
            Gate::And(Parameter::Value(1), "cc"), parse_and("1", "cc")
        );
    }

    #[test]
    fn gates() {
        assert_eq!(Gate::Or("aa", "bb"), parse_gate("aa OR bb"));
        assert_eq!(
            Gate::And(Parameter::Label("aa"), "bb"), parse_gate("aa AND bb")
        );
        assert_eq!(
            Gate::And(Parameter::Value(0), "bb"), parse_gate("0 AND bb")
        );
        assert_eq!(Gate::Not("bb"), parse_gate("NOT bb"));
        assert_eq!(
            Gate::None(Parameter::Label("bb")), parse_gate("bb")
        );
        assert_eq!(
            Gate::None(Parameter::Value(12345)), parse_gate("12345")
        );
        assert_eq!(Gate::Lshift("aa", 15), parse_gate("aa LSHIFT 15"));
        assert_eq!(Gate::Rshift("aa", 3), parse_gate("aa RSHIFT 3"));
    }

    #[test]
    fn lines() {
        assert_eq!(
            ("x", Gate::None(Parameter::Value(123))), parse_line("123 -> x")
        );
        assert_eq!(
            ("d", Gate::And(Parameter::Label("x"), "y")), parse_line("x AND y -> d")
        );
        assert_eq!(
            ("g", Gate::Rshift("y", 2)), parse_line("y RSHIFT 2 -> g")
        );
    }
}
