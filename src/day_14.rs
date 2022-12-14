use advent_of_code_helpers::line_iterator::LineIterator;
use advent_of_code_helpers::vec2d::Vec2D;
use aoc_runner_derive::aoc;
use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

type Output = usize;

#[derive(Debug, Copy, Clone)]
struct Coordinate(isize, isize);

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Fix this sometime in the vec2d, I might have mixed up my y and x
        let (column, row) = s.split_once(',').unwrap();

        Ok(Self(
            row.trim().parse().unwrap(),
            column.trim().parse().unwrap(),
        ))
    }
}

#[derive(Eq, PartialEq)]
enum Content {
    Air,
    Rock,
    Sand,
}

impl Debug for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Content::Air => ".",
                Content::Rock => "#",
                Content::Sand => "o",
            }
        )
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::Air
    }
}

fn apply_for_coordinate_line(
    start: Coordinate,
    end: Coordinate,
    mut apply: impl FnMut(Coordinate),
) {
    // Vertically
    for x in min(start.0, end.0)..=max(start.0, end.0) {
        let a = Coordinate(x, start.1);
        let b = Coordinate(x, end.1); // TODO: Why am I even generating B?
        if (a.1 - b.1).abs() > 0 {
            continue;
        }

        apply(a)
    }

    // Vertically
    for y in min(start.1, end.1)..=max(start.1, end.1) {
        let a = Coordinate(start.0, y);
        let b = Coordinate(end.0, y); // TODO: Why am I even generating B?
        if (a.0 - b.0).abs() > 0 {
            continue;
        }

        apply(a)
    }
}

fn generate(input: &str) -> Vec2D<Content> {
    let mut grid: Vec2D<Content> = Vec2D::from(vec![], 0, 0);

    for line in LineIterator::from(input) {
        let split = line
            .split("->")
            .map(|x| x.parse::<Coordinate>().unwrap())
            .collect::<Vec<_>>();

        for window in split.windows(2) {
            let start = window[0];
            let end = window[1];

            apply_for_coordinate_line(start, end, |coordinate| {
                *grid.growing_at_mut(coordinate.0, coordinate.1) = Content::Rock;
            });
        }
    }

    grid
}

#[aoc(day14, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut grid = generate(input);

    let how_deep = grid.height();
    let mut sand_placed = 0;

    while let Some(rest_point) = simulate_falling_sand(&mut grid, how_deep as isize) {
        *grid.at_mut_unchecked(rest_point.1, rest_point.0) = Content::Sand;
        sand_placed += 1;
    }

    sand_placed
}

#[aoc(day14, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut grid = generate(input);

    let how_deep = grid.height();
    let mut sand_placed = 0;

    let width = grid.width() + 1000;

    // Place an "infinite" line of rocks
    // Holy shit, reversing the iterator gave a 50% increase in performance (kinda logical since it only allocates once)
    for i in ((0 as isize)..(width as isize)).rev() {
        *grid.growing_at_mut(how_deep as isize + 1, i) = Content::Rock;
    }

    while let Some(rest_point) = simulate_falling_sand(&mut grid, (how_deep + 1) as isize) {
        *grid.at_mut_unchecked(rest_point.1, rest_point.0) = Content::Sand;
        sand_placed += 1;
    }

    sand_placed
}

fn simulate_falling_sand(grid: &mut Vec2D<Content>, max_y: isize) -> Option<Coordinate> {
    let mut sand = Coordinate(500, 0);

    loop {
        // Into the abyss we go
        if sand.1 > max_y {
            return None;
        }

        if grid.growing_at_mut(sand.1 + 1, sand.0) == &Content::Air {
            // Find directly below
            sand.1 += 1;
            continue;
        } else if grid.at_unchecked(sand.1 + 1, sand.0 - 1) == &Content::Air {
            // Find down left
            sand.1 += 1;
            sand.0 -= 1;
            continue;
        } else if grid.at_unchecked(sand.1 + 1, sand.0 + 1) == &Content::Air {
            // Find down right
            sand.1 += 1;
            sand.0 += 1;
            continue;
        } else {
            // Resting time
            break;
        }
    }

    if grid.at_unchecked(sand.1, sand.0) == &Content::Air {
        return Some(sand);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
    "#;

    #[test]
    fn test_part_1() {
        let expected = 24;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 93;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
