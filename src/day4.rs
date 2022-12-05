use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Section(u32, u32);

impl Section {
    pub fn intersection(&self, s2: &Section) -> Option<Section> {
        match (self, s2) {
            (s1, s2) if s1.1 < s2.0 || s2.1 < s1.0 => None,
            _ => Some(Self(cmp::max(self.0, s2.0), cmp::min(self.1, s2.1))),
        }
    }
    pub fn size(&self) -> u32 {
        self.1 - self.0
    }
}

impl From<&str> for Section {
    fn from(input: &str) -> Self {
        let mut split = input.split('-');
        Self(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    }
}

pub struct Pair(Section, Section);

impl Pair {
    pub fn inter(&self) -> Option<Section> {
        self.0.intersection(&self.1)
    }
    pub fn has_full_inter(&self) -> bool {
        match self.inter() {
            Some(s) => s.size() == cmp::min(self.0.size(), self.1.size()),
            None => false,
        }
    }

    pub fn has_inter(&self) -> bool {
        match self.inter() {
            Some(_) => true,
            None => false,
        }
    }
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let mut split = input.split(',');
        Self(
            Section::from(split.next().unwrap()),
            Section::from(split.next().unwrap()),
        )
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input.trim().lines().map(|line| line.into()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(pairs: &Vec<Pair>) -> usize {
    pairs.iter().filter(|pair| pair.has_full_inter()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(pairs: &Vec<Pair>) -> usize {
    pairs.iter().filter(|pair| pair.has_inter()).count()
}
