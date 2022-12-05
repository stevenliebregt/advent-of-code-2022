use crate::utils::LineIterator;
use aoc_runner_derive::aoc;

type Output = usize;

trait Score {
    fn score(&self) -> usize;
}

enum RoundResult {
    Lose,
    Tie,
    Win,
}

impl Score for RoundResult {
    #[inline]
    fn score(&self) -> usize {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Tie => 3,
            RoundResult::Win => 6,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    #[inline]
    fn from(byte: &u8) -> Self {
        match byte {
            &b'A' | &b'X' => Hand::Rock,
            &b'B' | &b'Y' => Hand::Paper,
            &b'C' | &b'Z' => Hand::Scissors,
            _ => panic!("Invalid byte input: {:?}", byte),
        }
    }

    /// Get the [`Hand`] this hand would win against
    fn wins_against(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    /// Get the [`Hand`] this hand would lose against
    fn loses_against(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl Score for Hand {
    #[inline]
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut score = 0;

    iterate(LineIterator::from(input), |opponent, you| {
        let opponent_hand = Hand::from(opponent);
        let your_hand = Hand::from(you);

        score += your_hand.score();
        score += round_result(&opponent_hand, &your_hand).score();
    });

    score
}

#[aoc(day2, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut score = 0;

    iterate(LineIterator::from(input), |opponent, guide| {
        let opponent_hand = Hand::from(opponent);
        let (your_hand, round_score) = match guide {
            // Need to lose
            &b'X' => (opponent_hand.wins_against(), 0),
            // Need to tie
            &b'Y' => (opponent_hand.clone(), 3),
            // Need to win
            &b'Z' => (opponent_hand.loses_against(), 6),
            _ => unreachable!(),
        };

        score += your_hand.score();
        score += round_score;
    });

    score
}

fn iterate(iter: LineIterator, mut f: impl FnMut(&u8, &u8)) {
    for line in iter {
        let bytes = line.as_bytes();
        f(&bytes[0], &bytes[2])
    }
}

fn round_result(opponent: &Hand, you: &Hand) -> RoundResult {
    if opponent == you {
        return RoundResult::Tie;
    }

    if &opponent.wins_against() == you {
        return RoundResult::Lose;
    }

    RoundResult::Win
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
A Y
B X
C Z
    "#;

    #[test]
    fn test_part_1() {
        let expected = 15;

        assert_eq!(expected, solve_part_1(INPUT.trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 12;

        assert_eq!(expected, solve_part_2(INPUT.trim()));
    }
}
