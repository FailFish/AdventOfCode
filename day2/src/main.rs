use std::{fs::File, io::{BufReader, BufRead, self}};

#[derive(Debug, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn new(s: &str) -> Self {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissor,
            _ => panic!(),
        }
    }

    fn compare(&self, opponent: &Choice) -> usize {
        use Choice::*;
        match (self, opponent) {
            (Rock, Scissor) | (Scissor, Paper) | (Paper, Rock) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        }
    }

    fn own_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn score(&self, opponent: &Choice) -> usize {
        let my_score = self.own_score();
        let win_score = self.compare(&opponent);
        println!("{my_score} + {win_score} = {}", my_score + win_score);
        my_score + win_score
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn new(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }

    fn score(&self, opponent: &Choice) -> usize {
        use Choice::*;
        let rule = vec![(Rock, Scissor), (Scissor, Paper), (Paper, Rock)];
        match self {
            Self::Win => 6 + rule.iter().find(|&r| r.1 == *opponent).unwrap().0.own_score(),
            Self::Draw => 3 + opponent.own_score(),
            Self::Lose => 0 + rule.iter().find(|&r| r.0 == *opponent).unwrap().1.own_score(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);
    let mut total = 0;
    let mut predicted_total = 0;

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf)? {
            0 => break,
            _ => {
                let mut tokens = buf.split_whitespace();
                let first_letter = tokens.next().unwrap();
                let second_letter = tokens.next().unwrap();
                let enemy = Choice::new(first_letter);
                let mine = Choice::new(second_letter);
                let prediction = RoundResult::new(second_letter);

                total += mine.score(&enemy);
                predicted_total += prediction.score(&enemy);
            },
        }
    }
    println!("first rule: {total}, second rule: {predicted_total}");
    Ok(())
}
