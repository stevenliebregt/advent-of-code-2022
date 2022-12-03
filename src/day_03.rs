use std::collections::HashSet;
use std::io::BufRead;
use crate::utils::reader_for_day;

pub fn run() {
    part_1();
    part_2();
}

fn part_1() {
    let mut buffer = String::new();

    let mut reader = reader_for_day(3);

    let mut score = 0;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let input = buffer.trim();
        let compartment_a = &input[..input.len() / 2];
        let compartment_b = &input[input.len() / 2..];

        println!("rucksack = \n\t> {}\n\t> {}", &compartment_a, &compartment_b);
        let duplicate_u8 = find_duplicate_char(&compartment_a, &compartment_b).unwrap();
        score += u8_to_score(duplicate_u8);
    }

    dbg!(&score);
}

fn part_2() {
    let mut buffer = String::new();

    let mut reader = reader_for_day(3);

    let mut score = 0;

    let mut elves: Vec<Vec<u8>> = Vec::with_capacity(3);

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let input = buffer.trim();
        elves.push(input.as_bytes().to_vec());

        if elves.len() == 3 {
            let badge = find_badge(&elves);
            score += u8_to_score(&badge);
            elves.clear();
        }
    }

    dbg!(&score);
}

fn find_badge(rucksacks: &[Vec<u8>]) -> u8 {
    let a: HashSet<u8> = rucksacks[0].to_vec().into_iter().collect();
    let b: HashSet<u8> = rucksacks[1].to_vec().into_iter().collect();
    let c: HashSet<u8> = rucksacks[2].to_vec().into_iter().collect();

    let ab_intersection = a.intersection(&b);
    let ab: HashSet<u8> = ab_intersection.cloned().collect();

    let abc_intersection = ab.intersection(&c);

    let mut result: Vec<&u8> = abc_intersection.collect();

    assert_eq!(1, result.len());

    result.remove(0).clone()
}

fn u8_to_score(byte: &u8) -> usize {
    match byte {
        x if x >= &b'A' && x <= &b'Z' => {
            ((x - b'A') + 27) as usize
        },
        x if x >= &b'a' && x <= &b'z' => {
            ((x - b'a') + 1) as usize
        }
        _ => unreachable!()
    }
}

fn find_duplicate_char<'b>(a: &str, b: &'b str) -> Option<&'b u8> {
    let pile: HashSet<&u8> = a.as_bytes().into_iter().collect();

    for byte in b.as_bytes() {
        if pile.contains(byte) {
            return Some(byte);
        }
    }

    None // Should not happen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_convert_scores() {
        assert_eq!(1, u8_to_score(&b'a'));
        assert_eq!(2, u8_to_score(&b'b'));
        assert_eq!(26, u8_to_score(&b'z'));

        assert_eq!(27, u8_to_score(&b'A'));
        assert_eq!(52, u8_to_score(&b'Z'));
    }
}