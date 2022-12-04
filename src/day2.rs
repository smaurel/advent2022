use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(c: char) -> Move {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Move {
    pub fn losing_move(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    pub fn winning_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    pub fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(c: char) -> Outcome {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

impl Outcome {
    pub fn score(&self) -> u8 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    p1_move: Move,
    p2_move: Move,
    desired_outcome: Outcome,
}

impl Round {
    fn from_line(line: &str) -> Self {
        let chars: Vec<char> = line.split(' ').map(|e| e.chars().next().unwrap()).collect();
        Self {
            p1_move: Move::from(chars[0]),
            p2_move: Move::from(chars[1].clone()),
            desired_outcome: Outcome::from(chars[1]),
        }
    }

    fn outcome_from_plays(&self) -> Outcome {
        match (self.p1_move, self.p2_move) {
            (Move::Rock, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (p1_move, p2_move) if p2_move.score() > p1_move.score() => Outcome::Win,
            (p1_move, p2_move) if p2_move.score() == p1_move.score() => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    fn my_score_from_plays(&self) -> u8 {
        self.outcome_from_plays().score() + self.p2_move.score()
    }

    fn move_to_do(&self) -> Move {
        match self.desired_outcome {
            Outcome::Loss => self.p1_move.losing_move(),
            Outcome::Win => self.p1_move.winning_move(),
            Outcome::Draw => self.p1_move.clone(),
        }
    }

    fn score_from_desired_outcome(&self) -> u8 {
        self.desired_outcome.score() + self.move_to_do().score()
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .trim()
        .lines()
        .map(|line| Round::from_line(line))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(rounds: &Vec<Round>) -> u32 {
    rounds
        .iter()
        .map(|round| u32::from(round.my_score_from_plays()))
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(rounds: &Vec<Round>) -> u32 {
    rounds
        .iter()
        .map(|round| u32::from(round.score_from_desired_outcome()))
        .sum()
}
