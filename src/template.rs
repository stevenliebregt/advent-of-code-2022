use aoc_runner_derive::aoc;

type Output = usize;

#[aoc(__template__, part1)]
pub fn solve_part_1(input: &str) -> Output {
    todo!("Implement part 1")
}

#[aoc(__template__, part2)]
pub fn solve_part_2(input: &str) -> Output {
    todo!("Implement part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"

    "#;

    #[test]
    fn test_part_1() {
        let expected = 1;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 1;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}