use std::{collections::HashMap, fs};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

// impl Move {
//     fn exec(&self, stacks: &mut HashMap<usize, Vec<char>>) {
//         for i in 0..self.quantity {
//             let e = stacks.entry(self.from);
//             e.
//         }
//     }
// }

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let mut split_input = input.split(' ');
        Self {
            quantity: split_input.nth(1).unwrap().parse().unwrap(),
            from: split_input.nth(1).unwrap().parse().unwrap(),
            to: split_input.nth(1).unwrap().parse().unwrap(),
        }
    }
}

// #[aoc_generator(day5)]
pub fn parse_input_generator(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut split = input.split("\n\n");
    let stack_lines = split.next().unwrap().lines().rev();
    let stack_chars = stack_lines
        .skip(1)
        .map(|line| line.chars().skip(1).step_by(4));
    let mut stacks = vec![];
    stack_chars.for_each(|line| stacks = push_line(stacks, line));

    let moves = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.into())
        .collect();

    (stacks, moves)
}

pub fn push_line(stacks: Vec<Vec<char>>, line: impl Iterator<Item = char>) -> Vec<Vec<char>> {
    line.enumerate()
        .filter(|(_, c)| c != &' ')
        .for_each(|(i, c)| {
            let mut stack = stacks.get(i).unwrap_or(&&mut vec![]);
            *stack.push(c);
        });

    stacks
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (stacks, moves) = parse_input_generator(input);
    println!("{:#?}", stacks);
    // println!("{:#?}", moves);
    5
}

// #[aoc(day5, part2)]
// pub fn solve_part2(pairs: &Vec<Pair>) -> usize {
//     pairs.iter().filter(|pair| pair.has_inter()).count()
// }
