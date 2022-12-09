use crate::utils::ParsingLineIterator;
use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

type Output = usize;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(format!("Invalid character: {s}")),
        }
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    amount: i32,
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_at(1);

        Ok(Self {
            direction: direction.trim().parse().unwrap(),
            amount: amount.trim().parse().unwrap(),
        })
    }
}

fn process_direction(head: &mut Position, tails: &mut [Position], direction: Direction) {
    match direction {
        Direction::Up => process_move_multiple(head, tails, (0, -1)),
        Direction::Down => process_move_multiple(head, tails, (0, 1)),
        Direction::Right => process_move_multiple(head, tails, (1, 0)),
        Direction::Left => process_move_multiple(head, tails, (-1, 0)),
    }
}

fn process_move(head: &mut Position, tail: &mut Position, movement_x_y: (i32, i32)) {
    head.x += movement_x_y.0;
    head.y += movement_x_y.1;

    follow_head(*head, tail);
}

fn process_move_multiple(head: &mut Position, tails: &mut [Position], movement_x_y: (i32, i32)) {
    head.x += movement_x_y.0;
    head.y += movement_x_y.1;

    follow_head(*head, &mut tails[0]);

    for i in 1..tails.len() {
        follow_head(tails[i - 1], &mut tails[i]);
    }
}

fn follow_head(head: Position, tail: &mut Position) {
    if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
        match head.x.cmp(&tail.x) {
            Ordering::Less => tail.x -= 1,
            Ordering::Equal => {}
            Ordering::Greater => tail.x += 1,
        }

        match head.y.cmp(&tail.y) {
            Ordering::Less => tail.y -= 1,
            Ordering::Equal => {}
            Ordering::Greater => tail.y += 1,
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut head = Position::default();
    let mut tails = vec![Position::default()];

    let mut seen: HashSet<Position> = HashSet::with_capacity(2000);

    for motion in ParsingLineIterator::<Motion>::from(input) {
        for _ in 0..motion.amount {
            process_direction(&mut head, &mut tails, motion.direction);

            seen.insert(tails[0]);
        }
    }

    seen.len()
}

#[aoc(day9, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut head = Position::default();
    let mut tails = vec![Position::default(); 10];

    let mut seen: HashSet<Position> = HashSet::with_capacity(2000);

    for motion in ParsingLineIterator::<Motion>::from(input) {
        for _ in 0..motion.amount {
            process_direction(&mut head, &mut tails, motion.direction);

            seen.insert(tails[8]);
        }
    }

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "#;

    #[test]
    fn test_part_1() {
        let expected = 13;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 1;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
