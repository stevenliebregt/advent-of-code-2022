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
        if value < self.values[2] {
            return; // No chance
        }

        if value > self.values[0] {
            self.values[2] = self.values[1];
            self.values[1] = self.values[0];
            self.values[0] = value;
        } else if value > self.values[1] {
            self.values[2] = self.values[1];
            self.values[1] = value;
        } else if value > self.values[2] {
            self.values[2] = value;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#;

        let expected = &24000;

        assert_eq!(expected, solve_part_1(input.trim()).highest());
    }

    #[test]
    fn test_part_2() {
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#;

        let expected = 45000;

        assert_eq!(expected, solve_part_2(input.trim()).sum());
    }
}
