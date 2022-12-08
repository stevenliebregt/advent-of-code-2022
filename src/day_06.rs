use aoc_runner_derive::aoc;

type Output = usize;

#[aoc(day6, part1)]
pub fn solve_part_1(input: &str) -> Output {
    find_packet_position(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part_2(input: &str) -> Output {
    find_packet_position(input, 14)
}

fn find_packet_position(input: &str, packet_size: usize) -> usize {
    input
        .as_bytes()
        .windows(packet_size)
        .position(all_unique_bytes)
        .expect("No solution")
        + packet_size
}

fn all_unique_bytes(mut bytes: &[u8]) -> bool {
    // Keep splitting first char
    while let Some((first, rest)) = bytes.split_first() {
        if rest.contains(first) {
            return false;
        }

        bytes = rest;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    fn test_part_1() {
        let expected = 7;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 19;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
