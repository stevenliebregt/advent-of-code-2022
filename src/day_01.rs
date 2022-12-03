use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::fmt::{Display, Formatter};

type Output = HighestStore;

#[derive(Default)]
pub struct HighestStore {
    values: [i32; 3],
}

impl HighestStore {
    fn try_add(&mut self, value: i32) {
        for i in 0..3 {
            if value > self.values[i] {
                // Shift other values
                for j in i + 1..3 {
                    self.values[j] = self.values[i];
                }
                self.values[i] = value;
                return;
            }
        }
    }

    fn highest(&self) -> &i32 {
        &self.values[0]
    }

    fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}

impl Display for HighestStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HighestStore")
            .field("highest", &self.highest())
            .field("sum", &self.sum())
            .finish()
    }
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &str) -> Output {
    solve(LineIterator::from(input))
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &str) -> Output {
    solve(LineIterator::from(input))
}

fn solve(input: LineIterator) -> HighestStore {
    let mut store = HighestStore::default();
    let mut current_elf_calories = 0;

    for line in input {
        if line.is_empty() {
            // Flush the current elf and reset calories
            store.try_add(current_elf_calories);
            current_elf_calories = 0;
            continue;
        }

        current_elf_calories += line.parse::<i32>().expect("Could not parse");
    }

    // Flush last elf
    store.try_add(current_elf_calories);

    store
}
