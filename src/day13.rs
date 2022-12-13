use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Vec<Packet> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[derive(PartialEq, Eq)]
pub enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        let without_brackets = input[1..input.len() - 1].split(",").collect_vec();
        if without_brackets.len() == 1 {
            Packet::Value(without_brackets[0].parse::<usize>().unwrap())
        } else {
            Packet::List(
                without_brackets
                    .into_iter()
                    .map(|el| Packet::from(el))
                    .collect_vec(),
            )
        }
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(packets: &Vec<Packet>) -> usize {
    // let mut head = Knot::new(1);
    // for mv in moves {
    //     head.move_with(mv);
    // }
    // head.get_tail().followed_positions.len()
    5
}
