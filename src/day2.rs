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
    pub fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    p1_move: Move,
    p2_move: Move,
}

impl Round {
    fn new(opponent_move: Move, my_move: Move) -> Self {
        Round {
            p1_move: opponent_move,
            p2_move: my_move,
        }
    }
    fn my_score(&self) -> u8 {
        let win_points = match (self.p1_move, self.p2_move) {
            (Move::Rock, Move::Scissors) => 0,
            (Move::Scissors, Move::Rock) => 6,
            (p1_move, p2_move) if p2_move.score() > p1_move.score() => 6,
            (p1_move, p2_move) if p2_move.score() == p1_move.score() => 3,
            _ => 0,
        };
        self.p2_move.score() + win_points
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .trim()
        .lines()
        .map(|line| line.split(' '))
        .map(|line| {
            line.map(|move_str| Move::from(move_str.chars().next().unwrap()))
                .collect::<Vec<Move>>()
        })
        .map(|line| Round::new(line[0], line[1]))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(|round| u32::from(round.my_score())).sum()
}

// #[aoc(day2, part2)]
// pub fn solve_part2(rations: &Vec<u32>) -> u32 {
//     let mut sorted_rations = rations.clone();
//     sorted_rations.sort();
//     sorted_rations[sorted_rations.len() - 3..].iter().sum()
// }
