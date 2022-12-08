use crate::utils::{LineIterator, LineIteratorSettings, TrimMode};
use aoc_runner_derive::aoc;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

type Output = String;

#[derive(Default)]
struct CargoHold {
    stacks: Vec<VecDeque<u8>>,
}

impl CargoHold {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stacks: vec![VecDeque::default(); capacity],
        }
    }

    pub fn add_to_stack(&mut self, identifier: usize, value: u8) {
        self.stacks[identifier].push_front(value);
    }

    fn operate_9000(&mut self, operation: MoveOperation) {
        let mut temporary: Vec<u8> = Vec::with_capacity(operation.amount);

        // Fill temporary
        {
            let stack = &mut self.stacks[operation.from];

            for _ in 0..operation.amount {
                temporary.push(stack.pop_back().unwrap());
            }
        }

        self.stacks[operation.to].extend(temporary);
    }

    fn operate_9001(&mut self, operation: MoveOperation) {
        let mut temporary: VecDeque<u8> = VecDeque::with_capacity(operation.amount);

        // Fill temporary
        {
            let stack = &mut self.stacks[operation.from];

            for _ in 0..operation.amount {
                temporary.push_front(stack.pop_back().unwrap());
            }
        }

        self.stacks[operation.to].extend(temporary);
    }

    pub fn get_tops(&self) -> String {
        let mut output = String::with_capacity(self.stacks.len());

        for stack in &self.stacks {
            output.push(*stack.back().unwrap() as char);
        }

        output
    }
}

impl Debug for CargoHold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("CargoHold");

        for (id, stack) in self.stacks.iter().enumerate() {
            debug.field(
                &format!("{id}"),
                &{
                    let mut string = String::new();

                    for value in stack {
                        string.push_str(&format!("[{}] ", std::str::from_utf8(&[*value]).unwrap()))
                    }

                    string
                }
                .trim(),
            );
        }

        debug.finish()
    }
}

struct MoveOperation {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for MoveOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<_>>();

        Ok(Self {
            amount: split[1].parse::<usize>().unwrap(),
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
        })
    }
}

struct MoveOperationIterator<'a> {
    lines: LineIterator<'a>,
}

impl<'a> MoveOperationIterator<'a> {
    fn new(input: &'a str) -> Self {
        MoveOperationIterator {
            lines: LineIterator::from(input),
        }
    }
}

impl Iterator for MoveOperationIterator<'_> {
    type Item = MoveOperation;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|line| line.parse::<MoveOperation>().expect("Not a move operation"))
    }
}

#[aoc(day5, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let starting_pos = input.find("move").expect("No move operations?");
    let (initial_state, move_operations) = input.split_at(starting_pos);

    let mut cargo_hold = initialize_cargo_hold(initial_state);

    for operation in MoveOperationIterator::new(move_operations) {
        cargo_hold.operate_9000(operation);
    }

    dbg!(&cargo_hold);

    cargo_hold.get_tops()
}

#[aoc(day5, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let starting_pos = input.find("move").expect("No move operations?");
    let (initial_state, move_operations) = input.split_at(starting_pos);

    let mut cargo_hold = initialize_cargo_hold(initial_state);

    for operation in MoveOperationIterator::new(move_operations) {
        cargo_hold.operate_9001(operation);
    }

    cargo_hold.get_tops()
}

#[inline(always)]
fn initialize_cargo_hold(initialization: &str) -> CargoHold {
    // Let find longest line first (probably faster than bounds checking each time)
    let longest_line_length = LineIterator::from(initialization)
        .map(|line| line.len() / 4)
        .max()
        .unwrap();
    let mut cargo_hold = CargoHold::with_capacity(longest_line_length + 1);

    // Parse initial state
    for line in LineIterator::from_settings(
        initialization,
        LineIteratorSettings {
            trim_mode: TrimMode::LineEndOnly,
        },
    ) {
        if line.trim().starts_with('1') {
            break;
        }

        for (index, byte) in line.bytes().enumerate() {
            if (b'A'..=b'Z').contains(&byte) {
                // Is a crate
                cargo_hold.add_to_stack(index / 4, byte);
            }
        }
    }

    cargo_hold
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"    [D]
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
