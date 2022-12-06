use aoc_runner_derive::aoc;
use std::collections::HashSet;

type Output = usize;

#[derive(Default, Debug)]
struct StartOfPacket {
    buffer: Vec<u8>,
}

impl StartOfPacket {
    fn add(&mut self, value: u8) {
        if self.buffer.len() < 4 {
            self.buffer.push(value);
        } else {
            // TODO: Ew
            self.buffer.remove(0);
            self.buffer.push(value);
        }
    }

    fn complete(&self) -> bool {
        self.buffer.len() == 4 && HashSet::<&u8>::from_iter(&mut self.buffer.iter()).len() == 4
    }
}

#[derive(Default, Debug)]
struct StartOfMessage {
    buffer: Vec<u8>,
}

impl StartOfMessage {
    fn add(&mut self, value: u8) {
        if self.buffer.len() < 14 {
            self.buffer.push(value);
        } else {
            // TODO: Ew
            self.buffer.remove(0);
            self.buffer.push(value);
        }
    }

    fn complete(&self) -> bool {
        self.buffer.len() == 14 && HashSet::<&u8>::from_iter(&mut self.buffer.iter()).len() == 14
    }
}

#[aoc(day6, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut start_packet = StartOfPacket::default();

    for (index, byte) in input.as_bytes().iter().enumerate() {
        start_packet.add(byte.clone());

        if start_packet.complete() {
            return index + 1;
        }
    }

    panic!("No solution")
}

#[aoc(day6, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut start_packet = StartOfMessage::default();

    for (index, byte) in input.as_bytes().iter().enumerate() {
        start_packet.add(byte.clone());

        if start_packet.complete() {
            return index + 1;
        }
    }

    panic!("No solution")
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
