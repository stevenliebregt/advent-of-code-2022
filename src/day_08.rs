use crate::utils::LineIterator;
use aoc_runner_derive::aoc;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

type Output = usize;

#[derive(Default)]
struct Matrix<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        self.inner.extend(iter);
    }

    fn set_shape(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn at(&self, row: usize, column: usize) -> &T {
        &self.inner[(row * self.width) + column]
    }

    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T> Debug for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut inner_format: Vec<String> = Vec::new();

        for chunk in self.inner.chunks(self.width) {
            inner_format.push(format!("{chunk:?}"));
        }

        f.debug_struct("Matrix")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("inner", &inner_format)
            .finish()
    }
}

#[aoc(day8, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let matrix = parse_matrix(input);

    let (width, height) = matrix.shape();

    let outer_trees = (width * 2) + ((height - 2) * 2);
    let mut trees_visible = 0;

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            if is_tree_visible(&matrix, i, j) {
                trees_visible += 1;
            }
        }
    }

    trees_visible + outer_trees
}

#[aoc(day8, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let matrix = parse_matrix(input);

    let (width, height) = matrix.shape();

    let mut max_scenic_score = 0;

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            let scenic_score = calculate_scenic_score(&matrix, i, j);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn parse_matrix(input: &str) -> Matrix<usize> {
    let mut matrix: Matrix<usize> = Matrix::default();

    let mut width = 0;
    let mut height = 0;

    for line in LineIterator::from(input) {
        matrix.extend(line.bytes().into_iter().map(|byte| byte as usize - 48));

        width = line.len();
        height += 1;
    }

    matrix.set_shape(width, height);

    matrix
}

// TODO: Inline this stuff?
fn is_tree_visible_row_range(
    forest: &Matrix<usize>,
    current_height: &usize,
    mut row_range: Range<usize>,
    column: usize,
) -> bool {
    !row_range.any(|row| forest.at(row, column) >= current_height)
}

fn is_tree_visible_column_range(
    forest: &Matrix<usize>,
    current_height: &usize,
    row: usize,
    mut column_range: Range<usize>,
) -> bool {
    !column_range.any(|column| forest.at(row, column) >= current_height)
}

fn is_tree_visible(forest: &Matrix<usize>, row: usize, column: usize) -> bool {
    let current_height = forest.at(row, column);

    // Check to the north
    if is_tree_visible_row_range(forest, current_height, 0..row, column) {
        return true;
    }

    // Check to the south
    if is_tree_visible_row_range(forest, current_height, row + 1..forest.height, column) {
        return true;
    }

    // Check to the west
    if is_tree_visible_column_range(forest, current_height, row, 0..column) {
        return true;
    }

    // Check to the east
    if is_tree_visible_column_range(forest, current_height, row, column + 1..forest.width) {
        return true;
    }

    false
}

fn calculate_scenic_score_row_range(
    forest: &Matrix<usize>,
    current_height: &usize,
    row_range: impl Iterator<Item = usize>,
    column: usize,
) -> usize {
    let mut score = 0;

    for row in row_range {
        score += 1;

        if forest.at(row, column) >= current_height {
            return score;
        }
    }

    score
}

fn calculate_scenic_score_column_range(
    forest: &Matrix<usize>,
    current_height: &usize,
    row: usize,
    column_range: impl Iterator<Item = usize>,
) -> usize {
    let mut score = 0;

    for column in column_range {
        score += 1;

        if forest.at(row, column) >= current_height {
            return score;
        }
    }

    score
}

fn calculate_scenic_score(forest: &Matrix<usize>, row: usize, column: usize) -> usize {
    let current_height = forest.at(row, column);

    let north = calculate_scenic_score_row_range(forest, current_height, (0..row).rev(), column);
    let south =
        calculate_scenic_score_row_range(forest, current_height, row + 1..forest.height, column);
    let west = calculate_scenic_score_column_range(forest, current_height, row, (0..column).rev());
    let east =
        calculate_scenic_score_column_range(forest, current_height, row, column + 1..forest.width);

    north * south * west * east
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
30373
25512
65332
33549
35390
    "#;

    #[test]
    fn test_matrix() {
        let mut matrix = Matrix::default();

        matrix.extend(
            vec![
                0, 1, 2, // Row 1
                3, 4, 5, // Row 2
                6, 7, 8, // Row 3
            ]
            .into_iter(),
        );

        matrix.set_shape(3, 3);
        assert_eq!((3, 3), matrix.shape());

        assert_eq!(&0, matrix.at(0, 0));
        assert_eq!(&2, matrix.at(0, 2));
        assert_eq!(&3, matrix.at(1, 0));
        assert_eq!(&7, matrix.at(2, 1));
    }

    #[test]
    fn test_part_1() {
        let expected = 21;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 8;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
