use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod day01;

enum RoundResult {
    Lose,
    Tie,
    Win,
}

fn main() {
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