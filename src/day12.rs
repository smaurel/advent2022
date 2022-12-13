use std::{
    cell::RefCell,
    collections::HashSet,
    hash::Hash,
    ops::{Add, Sub},
    rc::Rc,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Vec<Move> {
    input.lines().map(|line| line.into()).collect::<Vec<Move>>()
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(i32, i32);

impl Coordinate {
    pub fn is_adjacent_with(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= 1 && (self.1 - other.1).abs() <= 1
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

pub struct Grid {}

#[aoc(day12, part1)]
pub fn solve_part1(moves: &Vec<Move>) -> usize {
    let mut head = Knot::new(1);
    for mv in moves {
        head.move_with(mv);
    }
    head.get_tail().followed_positions.len()
}
