use std::{collections::HashMap, hash::Hash, vec};

use aoc_runner_derive::{aoc, aoc_generator};

// #[aoc_generator(day3)]
// pub fn input_generator(input: &str) -> Vec<Round> {
//     input
//         .trim()
//         .lines()
//         .map(|line| Round::from_line(line))
//         .collect()
// }

#[aoc(day3, part1)]
pub fn solve_part1(txt: &str) -> u32 {
    txt.trim().lines().map(count_line_priority).sum()
}

const UPPERCASE_A: u32 = 'A' as u32;
const LOWERCASE_A: u32 = 'a' as u32;
const UPPERCASE_OFFSET: u32 = UPPERCASE_A - 27;
const LOWERCASE_OFFSET: u32 = LOWERCASE_A - 1;

pub fn get_priority(c: char) -> u32 {
    println!("finding priority {}", c);
    match c as u32 {
        v if v < ('Z' as u32 + 1) => c as u32 - UPPERCASE_OFFSET,
        v if v < ('z' as u32 + 1) => c as u32 - LOWERCASE_OFFSET,
        _ => unreachable!(),
    }
}

pub fn count_line_priority(line: &str) -> u32 {
    let mut seen_characters: HashMap<char, u32> = HashMap::new();
    let last_first_part_char = (line.len() / 2) - 1;
    for (i, c) in line.chars().enumerate() {
        match (i, c) {
            (i, _) if i <= last_first_part_char => *seen_characters.entry(c).or_insert(0) += 1,
            (_, c) if seen_characters.contains_key(&c) => return get_priority(c),
            (_, _) => continue,
        };
    }
    unreachable!()
}

pub fn count_characters(line: &str) -> HashMap<char, u32> {
    let mut seen_characters: HashMap<char, u32> = HashMap::new();
    line.chars()
        .for_each(|c| *seen_characters.entry(c).or_insert(0) += 1);
    seen_characters
}

pub fn find_dupe(lines: &Vec<&str>) -> char {
    let mut value_count: HashMap<char, u32> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            let char_count = value_count.entry(c).or_insert(0);
            match char_count {
                2 if i == 2 => return c,
                char_count if char_count == &(i as u32) => *char_count += 1,
                _ => continue,
            };
        }
    }
    unreachable!()
}

#[aoc(day3, part2)]
pub fn solve_part2(txt: &str) -> u32 {
    let mut group: Vec<&str> = vec![];
    let mut total_priority: u32 = 0;
    for (i, line) in txt.trim().lines().enumerate() {
        group.push(line);
        if i % 3 == 2 {
            total_priority += get_priority(find_dupe(&group));
            group.clear();
        }
    }
    total_priority
}
