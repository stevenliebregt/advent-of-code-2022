use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;
use std::str::FromStr;

type Output = usize;

#[derive(Debug)]
struct Assignment(RangeInclusive<usize>);

impl Assignment {
    fn contained_one_way_or_another(&self, other: &Self) -> bool {
        (self.0.contains(&other.0.start()) && self.0.contains(&other.0.end()))
            || (other.0.contains(&self.0.start()) && other.0.contains(&self.0.end()))
    }

    fn has_overlap_with(&self, other: &Self) -> bool {
        self.0.contains(&other.0.start())
            || self.0.contains(&other.0.end())
            || other.0.contains(&self.0.start())
            || other.0.contains(&self.0.end())
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let from: usize = split.next().unwrap().parse().unwrap();
        let to: usize = split.next().unwrap().parse().unwrap();

        Ok(Self(from..=to))
    }
}

#[aoc(day4, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut count = 0;

    for pair in LineIterator::from(input) {
        let mut split = pair.split(',');
        let assignment_a: Assignment = split.next().unwrap().parse().unwrap();
        let assignment_b: Assignment = split.next().unwrap().parse().unwrap();

        if assignment_a.contained_one_way_or_another(&assignment_b) {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut count = 0;

    for pair in LineIterator::from(input) {
        let mut split = pair.split(',');
        let assignment_a: Assignment = split.next().unwrap().parse().unwrap();
        let assignment_b: Assignment = split.next().unwrap().parse().unwrap();

        if assignment_a.has_overlap_with(&assignment_b) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;

        let expected = 2;

        assert_eq!(expected, solve_part_1(input.trim()));
    }

    #[test]
    fn test_part_2() {
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;

        let expected = 4;

        assert_eq!(expected, solve_part_2(input.trim()));
    }
}
