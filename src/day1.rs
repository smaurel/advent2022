use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()))
        .map(|elf| elf.sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(rations: &Vec<u32>) -> u32 {
    *rations.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(rations: &Vec<u32>) -> u32 {
    let mut sorted_rations = rations.clone();
    sorted_rations.sort();
    sorted_rations[sorted_rations.len() - 3..].iter().sum()
}
