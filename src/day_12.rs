use advent_of_code_helpers::line_iterator::LineIterator;
use advent_of_code_helpers::vec2d::Vec2D;
use std::collections::VecDeque;

use aoc_runner_derive::aoc;

type Output = i32;

const MOVE_DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn bfs(grid: &Vec2D<u8>, start: (isize, isize), end: (isize, isize)) -> Option<i32> {
    let mut visited = Vec2D::new_sized_with(
        grid.positive_width(),
        grid.positive_height(),
        grid.negative_width(),
        grid.negative_height(),
        false,
    );

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((coordinate, path_length)) = queue.pop_front() {
        // We found the end
        if coordinate == end {
            return Some(path_length);
        }

        // Move
        for (move_row, move_column) in MOVE_DIRECTIONS {
            let next_coordinate = (coordinate.0 + move_row, coordinate.1 + move_column);

            let Some(next_value) = grid.at(next_coordinate.0, next_coordinate.1) else { continue; };
            let value = grid.at_unchecked(coordinate.0, coordinate.1) + 1;

            if &value >= next_value && !*visited.at_unchecked(next_coordinate.0, next_coordinate.1)
            {
                *visited.at_mut_unchecked(next_coordinate.0, next_coordinate.1) = true;
                queue.push_back((next_coordinate, path_length + 1))
            }
        }
    }

    None
}

fn parse(input: &str) -> (Vec2D<u8>, (isize, isize), (isize, isize)) {
    let line_iterator = LineIterator::from(input);
    let mut grid = Vec2D::from(vec![], 0, 0);

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in line_iterator.enumerate() {
        for (column, byte) in line.as_bytes().into_iter().enumerate() {
            let byte_to_insert = match byte {
                &b'S' => {
                    start = (row as isize, column as isize);
                    b'a'
                }
                &b'E' => {
                    end = (row as isize, column as isize);
                    b'z'
                }
                _ => *byte,
            };

            *grid.growing_at_mut(row as isize, column as isize) = byte_to_insert;
        }
    }

    (grid, start, end)
}

#[aoc(day12, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let (grid, start, end) = parse(input);

    bfs(&grid, start, end).unwrap()
}

#[aoc(day12, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let (grid, _, end) = parse(input);

    let mut current_minimum = i32::MAX;

    for row in 0..grid.positive_height() {
        for column in 0..grid.positive_width() {
            if grid.at_unchecked(row as isize, column as isize) == &b'a' {
                let Some(value) = bfs(&grid, (row as isize, column as isize), end) else { continue; };
                if value < current_minimum {
                    current_minimum = value;
                }
            }
        }
    }

    current_minimum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
    "#;

    #[test]
    fn test_part_1() {
        let expected = 31;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 29;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
