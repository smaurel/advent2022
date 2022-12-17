use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(i32, i32);

impl Coordinate {
    pub fn coords_between(&self, other: &Self) -> Vec<Coordinate> {
        let vector = other - self;
        let mut res = vec![];
        let max = if vector.0 != 0 { vector.0 } else { vector.1 };
        for i in 0..max.abs() + 1 {
            res.push(Coordinate(
                if vector.0 == 0 {
                    self.0
                } else if vector.0 < 0 {
                    self.0 - i
                } else {
                    self.0 + i
                },
                if vector.1 == 0 {
                    self.1
                } else if vector.1 < 0 {
                    self.1 - i
                } else {
                    self.1 + i
                },
            ));
        }
        res
    }
}

impl<'a, 'b> Add<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, other: &'b Coordinate) -> Coordinate {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a, 'b> Sub<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn sub(self, other: &'b Coordinate) -> Coordinate {
        Coordinate(self.0 - other.0, self.1 - other.1)
    }
}

impl From<&str> for Coordinate {
    fn from(input: &str) -> Self {
        let as_vec = input.split(',').collect_vec();
        Coordinate(as_vec[0].parse().unwrap(), as_vec[1].parse().unwrap())
    }
}

const FALLING_DIRECTIONS: &[Coordinate] = &[Coordinate(0, 1), Coordinate(-1, 1), Coordinate(1, 1)];

#[derive(Debug)]
pub struct SandGrid {
    rocks: HashSet<Coordinate>,
    grains_fallen: usize,
    lowest_height: i32,
}

impl SandGrid {
    pub fn fall(&mut self, from: Coordinate) -> Option<()> {
        if from.1 > self.lowest_height {
            return None;
        }
        let mut i = 0;
        while i < FALLING_DIRECTIONS.len() && self.rocks.contains(&(&from + &FALLING_DIRECTIONS[i]))
        {
            i += 1
        }

        if i == FALLING_DIRECTIONS.len() {
            if from == Coordinate(500, 0) {
                return None;
            }
            self.rocks.insert(from);
            return Some(());
        }
        return self.fall(&from + &FALLING_DIRECTIONS[i]);
    }

    pub fn add_grain(&mut self) -> Option<()> {
        let stopped = self.fall(Coordinate(500, 0));
        if stopped != None {
            self.grains_fallen += 1
        }
        return stopped;
    }

    pub fn add_floor(&mut self) {
        self.lowest_height += 2;
        for i in (500 - 10 * self.lowest_height)..(500 + 10 * self.lowest_height + 1) {
            self.rocks.insert(Coordinate(i, self.lowest_height));
        }
    }
}

pub fn input_generator(input: &str) -> SandGrid {
    let mut sand_grid = SandGrid {
        rocks: HashSet::new(),
        grains_fallen: 0,
        lowest_height: i32::MIN,
    };

    for line in input.lines() {
        let coords: Vec<Coordinate> = line.split(" -> ").map(|coord| coord.into()).collect_vec();
        for i in 0..coords.len() - 1 {
            for position in coords[i].coords_between(&coords[i + 1]) {
                sand_grid.rocks.insert(position.clone());
                if sand_grid.lowest_height < position.1 {
                    sand_grid.lowest_height = position.1;
                }
            }
        }
    }
    sand_grid
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut grid = input_generator(input);
    while let Some(()) = grid.add_grain() {}
    grid.grains_fallen
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut grid = input_generator(input);
    grid.add_floor();
    while let Some(()) = grid.add_grain() {}
    grid.grains_fallen + 1
}

#[cfg(test)]
mod test {
    use crate::day14::*;

    #[test]
    fn test_coords_between() {
        let expected = vec![
            Coordinate(502, 9),
            Coordinate(501, 9),
            Coordinate(500, 9),
            Coordinate(499, 9),
            Coordinate(498, 9),
            Coordinate(497, 9),
            Coordinate(496, 9),
            Coordinate(495, 9),
            Coordinate(494, 9),
        ];
        let got = Coordinate(502, 9).coords_between(&Coordinate(494, 9));
        assert_eq!(expected.len(), got.len());
        assert!(expected.iter().filter(|ex| got.contains(ex)).count() == expected.len());
    }
}
