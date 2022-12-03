use std::collections::HashSet;
use std::io::BufRead;
use crate::utils::reader_for_day;

pub fn run() {
    part_1();
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