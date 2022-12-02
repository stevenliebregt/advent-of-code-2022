use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

trait Score {
    fn score(&self) -> usize;
}

#[derive(Debug, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn try_from_opponent(opponent: &str) -> Result<Self, String> {
        match opponent {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(format!("Invalid input for hand: {:?}", opponent))
        }
    }

    /// Get the [`Hand`] this hand would win against
    fn wins_against(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    /// Get the [`Hand`] this hand would lose against
    fn loses_against(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        }
    }
}

impl Score for Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Tie,
    Win,
}

impl Score for RoundResult {
    fn score(&self) -> usize {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Tie => 3,
            RoundResult::Win => 6
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("resources/day_2.txt").expect("Could not open input");
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut score = 0;

    loop {
        buffer.clear();

        let bytes_read = reader.read_line(&mut buffer).expect("Failed to read");

        if bytes_read == 0 {
            break;
        }

        let mut split = buffer.trim().split(' ');
        let opponent = split.next().unwrap();
        let you = split.next().unwrap();

        let your_score = match you {
            "X" =>  1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!()
        };

        score += your_score;

        let round_score = match round_result(opponent, you) {
            RoundResult::Lose => 0,
            RoundResult::Tie => 3,
            RoundResult::Win => 6
        };

        score += round_score;
    }

    dbg!(&score);
}


fn part_2() {
    let file = File::open("resources/day_2.txt").expect("Could not open input");
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut score = 0;

    loop {
        buffer.clear();

        let bytes_read = reader.read_line(&mut buffer).expect("Failed to read");

        if bytes_read == 0 {
            break;
        }

        let mut split = buffer.trim().split(' ');
        let opponent = split.next().unwrap();
        let round_result_expected = split.next().unwrap();

        let you = match round_result_expected {
            // TODO: Isn't there some smart shifty way to do this?
            // Need to lose
            "X" =>  match opponent {
                "A" => "Z",
                "B" => "X",
                "C" => "Y",
                _ => unreachable!()
            },
            // Need to tie
            "Y" => match opponent {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => unreachable!()
            },
            // Need to win (shift 1 back?)
            "Z" => match opponent {
                "A" => "Y",
                "B" => "Z",
                "C" => "X",
                _ => unreachable!()
            },
            _ => unreachable!()
        };

        let your_score = match you {
            "X" =>  1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!()
        };

        score += your_score;

        let round_score = match round_result(opponent, you) {
            RoundResult::Lose => 0,
            RoundResult::Tie => 3,
            RoundResult::Win => 6
        };

        score += round_score;
    }

    dbg!(&score);
}

fn round_result(opponent: &str, you: &str) -> RoundResult {
    if opponent == you { // TODO: Only works if strings are parsed into something else, which I probably want to do
        return RoundResult::Tie;
    }

    match (opponent, you) {
        // Rock vs Rock
        ("A", "X") => RoundResult::Tie,
        // Rock vs Paper
        ("A", "Y") => RoundResult::Win,
        // Rock vs Scissor
        ("A", "Z") => RoundResult::Lose,
        // Paper vs Rock
        ("B", "X") => RoundResult::Lose,
        // Paper vs Paper
        ("B", "Y") => RoundResult::Tie,
        // Paper vs Scissor
        ("B", "Z") => RoundResult::Win,
        // Scissor vs Rock
        ("C", "X") => RoundResult::Win,
        // Scissor vs Paper
        ("C", "Y") => RoundResult::Lose,
        // Scissor vs Scissor
        ("C", "Z") => RoundResult::Tie,
        _ => {
            dbg!(opponent);
            dbg!(you);
            unreachable!();
        }
    }
}