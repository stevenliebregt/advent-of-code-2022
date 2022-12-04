use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::collections::HashSet;

type Output = usize;

#[aoc(day3, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut score = 0;

    for line in LineIterator::from(input) {
        let compartment_a = &line[..line.len() / 2];
        let compartment_b = &line[line.len() / 2..];

        let duplicate = find_duplicate(&compartment_a.as_bytes(), &compartment_b.as_bytes());
        score += u8_to_priority(duplicate);
    }

    score
}

#[aoc(day3, part1, alt = "find_duplicate_iter")]
pub fn solve_part_1_alt(input: &str) -> Output {
    let mut score = 0;

    for line in LineIterator::from(input) {
        let compartment_a = &line[..line.len() / 2];
        let compartment_b = &line[line.len() / 2..];

        let duplicate = find_duplicate_iter(&compartment_a.as_bytes(), &compartment_b.as_bytes());
        score += u8_to_priority(duplicate);
    }

    score
}

#[aoc(day3, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut score = 0;

    let mut elves: Vec<Vec<u8>> = Vec::with_capacity(3);

    for line in LineIterator::from(input) {
        elves.push(line.as_bytes().to_vec());

        if elves.len() == 3 {
            let badge = find_badge(&elves);
            score += u8_to_priority(badge);
            elves.clear();
        }
    }

    score
}

#[aoc(day3, part2, alt = "collect_and_chunks")]
pub fn solve_part_2_alt(input: &str) -> Output {
    let mut score = 0;

    for group in LineIterator::from(input)
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>()
        .chunks(3)
    {
        let badge = group[0]
            .iter()
            .find(|byte| group[1..].iter().all(|rucksack| rucksack.contains(byte)))
            .expect("No common");

        score += u8_to_priority(badge);
    }

    score
}

fn find_duplicate<'b>(a: &[u8], b: &'b [u8]) -> &'b u8 {
    // TODO: Rethink this (after benching, yeah this is a bad idea)
    let pile: HashSet<&u8> = a.into_iter().collect();

    for byte in b {
        if pile.contains(byte) {
            return byte;
        }
    }

    unreachable!()
}

fn find_duplicate_iter<'a>(a: &'a [u8], b: &[u8]) -> &'a u8 {
    a.iter()
        .find(|byte| b.contains(byte))
        .expect("No duplicate")
}

fn u8_to_priority(byte: &u8) -> usize {
    match byte {
        x if x >= &b'A' && x <= &b'Z' => ((x - b'A') + 27) as usize,
        x if x >= &b'a' && x <= &b'z' => ((x - b'a') + 1) as usize,
        _ => unreachable!(),
    }
}

fn find_badge(elves: &[Vec<u8>]) -> &u8 {
    elves[0]
        .iter()
        .find(|byte| elves[1..].iter().all(|rucksack| rucksack.contains(byte)))
        .expect("No common")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn test_part_1() {
        let expected = 157;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 70;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}