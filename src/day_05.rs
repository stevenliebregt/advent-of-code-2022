use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use aoc_runner_derive::aoc;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::utils::{LineIterator, LineIteratorSettings, TrimMode};

const MOVE_OP_PATTERN: &str = r#"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)"#;
static MOVE_OP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(MOVE_OP_PATTERN).unwrap());

type Output = String;

#[derive(Default)]
struct CargoHold {
    stacks: HashMap<usize, VecDeque<u8>>
}

impl CargoHold {
    pub fn add_to_stack(&mut self, identifier: usize, value: u8) {
        let stack = self.stacks.entry(identifier).or_insert_with(|| VecDeque::default());
        stack.push_front(value);
    }

    pub fn move_crates(&mut self, amount: usize, from: usize, to: usize) {
        let mut temp: Vec<u8> = Vec::default();

        // println!("\t: move {:?} from {:?} to {:?}", amount, from, to);

        {
            let origin = self.stacks.get_mut(&from).expect("No origin");

            for _ in 0..amount {
                let value = origin.pop_back().expect("Could not pop");
                temp.push(value);
            }
        }

        self.stacks.entry(to).and_modify(|v| v.extend(temp));

        // dbg!(&self);
    }

    pub fn move_crates_at_once(&mut self, amount: usize, from: usize, to: usize) {
        let mut temp: VecDeque<u8> = VecDeque::default();

        {
            let origin = self.stacks.get_mut(&from).expect("No origin");

            for _ in 0..amount {
                let value = origin.pop_back().expect("Could not pop");
                temp.push_front(value);
            }
        }

        self.stacks.entry(to).and_modify(|v| v.extend(temp));
    }

    pub fn get_tops(&self) -> String {
        let mut output = String::with_capacity(self.stacks.len());

        for i in 0..self.stacks.len() {
            output.push(self.stacks.get(&i).unwrap().back().unwrap().clone() as char);
        }

        output
    }
}

impl Debug for CargoHold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("CargoHold");

        let debug_format_line = |index: usize| {
            let mut string = String::new();
            for b in self.stacks.get(&index).unwrap() {
                string.push_str(&format!("[{}] ", std::str::from_utf8(&[b.clone()]).unwrap()));
            }
            string
        };

        for i in 0..self.stacks.len() {
            debug.field(&format!("{}", i), &debug_format_line(i));
        }

        debug.finish()
    }
}

#[aoc(day5, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let starting_pos = input.find("move").expect("No move operations?");
    let (initial_state, move_operations) = input.split_at(starting_pos);

    let mut cargo_hold = CargoHold::default();

    // Parse initial state
    for line in LineIterator::from_settings(initial_state, LineIteratorSettings { trim_mode: TrimMode::LineEndOnly }) {
        if line.trim().starts_with('1') {
            break;
        }

        for (index, byte) in line.bytes().enumerate() {
            if byte >= b'A' && byte <= b'Z' { // Is a crate
                cargo_hold.add_to_stack(index / 4, byte);
            }
        }
    }

    dbg!(&cargo_hold);

    for line in LineIterator::from(move_operations) {
        println!("Line = {:?}", line);

        let matches = MOVE_OP_REGEX.captures(line).unwrap();
        let amount: usize = matches.name("amount").unwrap().as_str().parse().unwrap();
        let from: usize = matches.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = matches.name("to").unwrap().as_str().parse().unwrap();

        cargo_hold.move_crates(amount, from - 1, to - 1);
    }

    cargo_hold.get_tops()
}

#[aoc(day5, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let starting_pos = input.find("move").expect("No move operations?");
    let (initial_state, move_operations) = input.split_at(starting_pos);

    let mut cargo_hold = CargoHold::default();

    // Parse initial state
    for line in LineIterator::from_settings(initial_state, LineIteratorSettings { trim_mode: TrimMode::LineEndOnly }) {
        if line.trim().starts_with('1') {
            break;
        }

        for (index, byte) in line.bytes().enumerate() {
            if byte >= b'A' && byte <= b'Z' { // Is a crate
                cargo_hold.add_to_stack(index / 4, byte);
            }
        }
    }

    dbg!(&cargo_hold);

    for line in LineIterator::from(move_operations) {
        println!("Line = {:?}", line);

        let matches = MOVE_OP_REGEX.captures(line).unwrap();
        let amount: usize = matches.name("amount").unwrap().as_str().parse().unwrap();
        let from: usize = matches.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = matches.name("to").unwrap().as_str().parse().unwrap();

        cargo_hold.move_crates_at_once(amount, from - 1, to - 1);
    }

    cargo_hold.get_tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_1() {
        let expected = String::from("CMZ");

        assert_eq!(expected, solve_part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        let expected = String::from("MCD");

        assert_eq!(expected, solve_part_2(INPUT));
    }
}