extern crate simple_error;

use simple_error::SimpleError;

use std::cmp::{Ord, Ordering, PartialEq};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Hand {
    us_move: Move,
    them_move: Move,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Move::Rock => match other {
                Move::Rock => Ordering::Equal,
                Move::Paper => Ordering::Less,
                Move::Scissors => Ordering::Greater,
            },
            Move::Paper => match other {
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
                Move::Rock => Ordering::Greater,
            },
            Move::Scissors => match other {
                Move::Scissors => Ordering::Equal,
                Move::Rock => Ordering::Less,
                Move::Paper => Ordering::Greater,
            },
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Move {
    fn outcom_with(&self, other: &Move) -> Outcome {
        match self.cmp(other) {
            Ordering::Less => Outcome::Loose,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Loose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl Hand {
    fn get_outcome(&self) -> Outcome {
        self.us_move.outcom_with(&self.them_move)
    }
    fn get_score(&self) -> u32 {
        self.us_move.score() + self.get_outcome().score()
    }
}

fn parse_hand_from_first_case(line: &str) -> Result<Hand, SimpleError> {
    let (them_move, us_move) = line.split_once(' ').unwrap();

    let us_move = match us_move {
        "X" => Ok(Move::Rock),
        "Y" => Ok(Move::Paper),
        "Z" => Ok(Move::Scissors),
        _ => Err(SimpleError::new("Failed to parse the us move")),
    }?;

    let them_move = match them_move {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        _ => Err(SimpleError::new("Failed to parse the us move")),
    }?;

    Ok(Hand { us_move, them_move })
}

fn parse_hand_from_second_case(line: &str) -> Result<Hand, SimpleError> {
    let (them_move, expected_outcome) = line.split_once(' ').unwrap();

    let expected_outcome = match expected_outcome {
        "X" => Ok(Outcome::Loose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err(SimpleError::new("Failed to parse the us move")),
    }?;

    let them_move = match them_move {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        _ => Err(SimpleError::new("Failed to parse the us move")),
    }?;

    let us_move = match expected_outcome {
        Outcome::Draw => them_move.clone(),
        Outcome::Loose => match them_move {
            Move::Rock => Move::Scissors,
            Move::Scissors => Move::Paper,
            Move::Paper => Move::Rock,
        },
        Outcome::Win => match them_move {
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock,
            Move::Paper => Move::Scissors,
        },
    };

    Ok(Hand { us_move, them_move })
}

fn get_hands_from_input_first_case(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_hand_from_first_case(line).unwrap())
        .collect()
}

fn get_hands_from_input_second_case(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_hand_from_second_case(line).unwrap())
        .collect()
}

fn get_scores(hands: &Vec<Hand>) -> Vec<u32> {
    hands.iter().map(|hand| hand.get_score()).collect()
}

pub fn puzzle_1(input: &str) -> u32 {
    let hands = get_hands_from_input_first_case(input);
    let scores = get_scores(&hands);

    scores.iter().sum()
}

pub fn puzzle_2(input: &str) -> u32 {
    let hands = get_hands_from_input_second_case(input);
    let scores = get_scores(&hands);

    scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hand_case_1() {
        let hand_str = "A Y";
        let hand = parse_hand_from_first_case(hand_str).unwrap();

        assert_eq!(hand.them_move, Move::Rock);
        assert_eq!(hand.us_move, Move::Paper);
        assert_eq!(hand.get_outcome(), Outcome::Win);
        assert_eq!(hand.get_score(), 8);
    }
    #[test]
    fn parse_hand_case_2() {
        let hand_str = "A Y";
        let hand = parse_hand_from_second_case(hand_str).unwrap();

        assert_eq!(hand.them_move, Move::Rock);
        assert_eq!(hand.us_move, Move::Rock);
        assert_eq!(hand.get_outcome(), Outcome::Draw);
        assert_eq!(hand.get_score(), 4);
    }
}
