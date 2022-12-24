use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        match stripped.as_str() {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        match stripped.as_str() {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut total = 0;
    for line in reader.lines() {
        let string = line.unwrap();
        let mut split_n = string.splitn(2, ' ');
        let ant: Hand = split_n.next().unwrap().parse().unwrap();
        let pro: Outcome = split_n.next().unwrap().parse().unwrap();
        total += match (ant, pro) {
            (Hand::Paper, p @ Outcome::Loss)
            | (Hand::Rock, p @ Outcome::Draw)
            | (Hand::Scissors, p @ Outcome::Win) => p as u32 + Hand::Rock as u32,
            (Hand::Scissors, p @ Outcome::Loss)
            | (Hand::Paper, p @ Outcome::Draw)
            | (Hand::Rock, p @ Outcome::Win) => p as u32 + Hand::Paper as u32,
            (Hand::Rock, p @ Outcome::Loss)
            | (Hand::Scissors, p @ Outcome::Draw)
            | (Hand::Paper, p @ Outcome::Win) => p as u32 + Hand::Scissors as u32,
        }
    }
    println!("{total}");
    Ok(())
}
