use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::str::FromStr;

type Output = usize;

struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    /// Checks whether the current assignment fully contains the other assignment, or the other
    /// way around.
    ///
    /// In this example `self` fully contains `other``:
    /// ```md
    /// .2345678.
    /// ...456...
    /// ```
    ///
    /// In this example `other` fully contains `self`:
    /// ```md
    /// ....56...
    /// ...4567..
    /// ```
    #[inline(always)]
    fn contained_one_way_or_another(&self, other: &Self) -> bool {
        // Check if we contain other
        (self.start <= other.start && self.end >= other.end)
        // And other way around
        || (other.start <= self.start && other.end >= self.end)
    }

    /// Checks whether the current assignment has any overlap with the other assignment
    #[inline(always)]
    fn has_overlap_with(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (other.start <= self.start && other.end >= self.start)
    }
}

impl FromStr for Assignment {
    type Err = String;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start: usize = split.next().unwrap().parse().unwrap();
        let end: usize = split.next().unwrap().parse().unwrap();

        Ok(Self { start, end })
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

    parameterized_test::create! { test_assignment_contained_one_way_or_another, input, {
        assert_eq!(input.2, input.0.contained_one_way_or_another(&input.1))
    }}

    test_assignment_contained_one_way_or_another! {
        contained: (
            Assignment { start: 3, end: 8 },
            Assignment { start: 4, end: 6 },
            true
        ),
        not_contained: (
            Assignment { start: 5, end: 8 },
            Assignment { start: 4, end: 6 },
            false
        ),
        contained_reverse: (
            Assignment { start: 4, end: 6 },
            Assignment { start: 3, end: 8 },
            true
        ),
        not_contained_reverse: (
            Assignment { start: 4, end: 6 },
            Assignment { start: 5, end: 8 },
            false
        ),
    }

    parameterized_test::create! { test_assignment_has_overlap_with, input, {
        assert_eq!(input.2, input.0.has_overlap_with(&input.1))
    }}

    test_assignment_has_overlap_with! {
        // ..34567..
        // ...45678.
        overlaps: (
            Assignment { start: 3, end: 7 },
            Assignment { start: 4, end: 8 },
            true
        ),
        // ...45678.
        // ..34567..
        overlaps_reverse: (
            Assignment { start: 4, end: 8 },
            Assignment { start: 3, end: 7 },
            true
        ),
        // .234.....
        // ...4567..
        overlaps_only_single: (
            Assignment { start: 2, end: 4 },
            Assignment { start: 4, end: 7 },
            true
        ),
        no_overlap: (
            Assignment { start: 2, end: 4 },
            Assignment { start: 5, end: 7 },
            false
        ),
        no_overlap_reversee: (
            Assignment { start: 5, end: 7 },
            Assignment { start: 2, end: 4 },
            false
        ),
    }

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
