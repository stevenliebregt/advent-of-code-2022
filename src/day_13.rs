use advent_of_code_helpers::line_iterator::LineIterator;
use aoc_runner_derive::aoc;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;

type Output = usize;

#[derive(Parser)]
#[grammar = "resources/day_13.pest"]
pub struct PacketParser;

#[derive(Debug, Clone)]
enum Packet {
    Int(u64),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Normal case, both ints
            (Packet::Int(int_left), Packet::Int(int_right)) => int_left.partial_cmp(int_right),
            // Normal case, both lists
            (Packet::List(list_left), Packet::List(list_right)) => {
                // Zip left and right and compare each value
                for (int_left, int_right) in list_left.iter().zip(list_right) {
                    match int_left.partial_cmp(int_right) {
                        Some(result) if result != Ordering::Equal => return Some(result),
                        _ => {}
                    }
                }

                // Compare lengths as fallback
                list_left.len().partial_cmp(&list_right.len())
            }
            // Special cases, one is list, other is int
            (Packet::Int(int_left), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*int_left)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Int(int_right)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Int(*int_right)]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn pest_int_to_u64(pest_int: Pair<Rule>) -> u64 {
    pest_int.as_span().as_str().parse::<u64>().unwrap()
}

fn pest_list_to_packet(pest_list: Pair<Rule>) -> Packet {
    let mut list: Vec<Packet> = vec![];

    for child in pest_list.into_inner() {
        match child.as_rule() {
            Rule::int => list.push(Packet::Int(pest_int_to_u64(child))),
            Rule::list => list.push(pest_list_to_packet(child)),
            _ => unreachable!(),
        }
    }

    Packet::List(list)
}

fn parse_line_as_list(line: &str) -> Packet {
    pest_list_to_packet(
        PacketParser::parse(Rule::list, line)
            .unwrap()
            .next()
            .unwrap(),
    )
}

#[aoc(day13, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut line_iterator = LineIterator::from(input);
    let mut ordered_count = 0;

    let mut index = 1;

    while let Some(line_left) = line_iterator.next() {
        let line_right = line_iterator.next().unwrap();

        let left = parse_line_as_list(line_left);
        let right = parse_line_as_list(line_right);

        if left < right {
            ordered_count += index;
        }

        // Consume blank line
        if line_iterator.next().is_none() {
            break;
        }

        index += 1;
    }

    ordered_count
}

#[aoc(day13, part2)]
pub fn solve_part_2(input: &str) -> Output {
    // TODO: Extend ParsingLineIterator to skip empty lines
    let mut packets: Vec<Packet> = vec![];

    for line in LineIterator::from(input) {
        if line.is_empty() {
            continue;
        }

        packets.push(parse_line_as_list(line));
    }

    // Push dividers
    let divider_a = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    packets.push(divider_a.clone());
    let divider_b = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(divider_b.clone());

    packets.sort();

    let index_divider_a = packets
        .iter()
        .position(|packet| packet == &divider_a)
        .unwrap();
    let index_divider_b = packets
        .iter()
        .position(|packet| packet == &divider_b)
        .unwrap();

    (index_divider_a + 1) * (index_divider_b + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
    "#;

    #[test]
    fn test_part_1() {
        let expected = 13;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 140;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
