use std::cmp;

use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn exec(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..cmp::min(self.quantity, stacks[self.from - 1].len()) {
            let e = stacks[self.from - 1].pop().unwrap();
            stacks[self.to - 1].push(e);
        }
    }

    fn exec_p2(&self, stacks: &mut Vec<Vec<char>>) {
        let starting_range = cmp::max(stacks[self.from - 1].len() - self.quantity, 0);
        let mut retrieved_containers = stacks[self.from - 1].split_off(starting_range);
        stacks[self.to - 1].append(&mut retrieved_containers);
    }
}

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
    stack_chars.for_each(|line| push_line(&mut stacks, line));

    let moves = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.into())
        .collect();

    (stacks, moves)
}

pub fn push_line(stacks: &mut Vec<Vec<char>>, line: impl Iterator<Item = char>) {
    line.enumerate()
        .filter(|(_, c)| c != &' ')
        .for_each(|(i, c)| {
            let stack = stacks.get(i);
            match stack {
                Some(_) => stacks[i].push(c),
                None => stacks.push(vec![c]),
            };
        });
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input_generator(input);
    moves.iter().for_each(|mv| mv.exec(&mut stacks));
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input_generator(input);
    moves.iter().for_each(|mv| mv.exec_p2(&mut stacks));
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
