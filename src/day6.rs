use aoc_runner_derive::aoc;
use std::collections::HashSet;

pub fn get_first_marker(line: &str, matching_chars: usize) -> usize {
    let mut char_set = HashSet::new();
    for i in 0..line.len() - matching_chars + 1 {
        char_set.clear();
        line[i..i + matching_chars].chars().for_each(|c| {
            char_set.insert(c);
        });
        if char_set.len() == matching_chars {
            return i + matching_chars;
        }
    }
    0
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| get_first_marker(line, 4))
        .next()
        .unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| get_first_marker(line, 14))
        .next()
        .unwrap()
}
