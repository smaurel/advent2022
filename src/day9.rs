use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
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

pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn get_vector(&self) -> Coordinate {
        match self {
            Self::Top => Coordinate(0, 1),
            Self::Right => Coordinate(1, 0),
            Self::Bottom => Coordinate(0, -1),
            Self::Left => Coordinate(-1, 0),
        }
    }
}

pub struct Move {
    direction: Direction,
    amount: usize,
}

impl Move {
    pub fn iter_vector(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (0..self.amount).map(|_| self.direction.get_vector())
    }
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let mut split = line.split_whitespace();
        Self {
            direction: match split.next().unwrap().chars().next().unwrap() {
                'U' => Direction::Top,
                'R' => Direction::Right,
                'D' => Direction::Bottom,
                'L' => Direction::Left,
                _ => unreachable!("found incorrect char for move parsing"),
            },
            amount: split.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

pub struct Head {
    position: Coordinate,
    tail: Tail,
}

impl Head {
    pub fn new() -> Self {
        return Self {
            position: Coordinate(0, 0),
            tail: Tail::new(),
        };
    }

    pub fn move_with(&mut self, mv: &Move) -> Vec<Coordinate> {
        let mut followed_positions: Vec<Coordinate> = vec![];
        for vector in mv.iter_vector() {
            self.position = &self.position + &vector;
            self.tail.follow(&self.position);
            followed_positions.push(self.tail.position.clone())
        }
        followed_positions
    }
}

pub struct Tail {
    position: Coordinate,
}

impl Tail {
    pub fn new() -> Self {
        return Self {
            position: Coordinate(0, 0),
        };
    }

    pub fn follow(&mut self, position: &Coordinate) {
        if self.position.is_adjacent_with(&position) {
            return;
        }

        let vec = position - &self.position;
        match vec {
            Coordinate(0, y) => {
                self.position = &self.position + &Coordinate(0, if y > 0 { y - 1 } else { y + 1 })
            }
            Coordinate(x, 0) => {
                self.position = &self.position + &Coordinate(if x > 0 { x - 1 } else { x + 1 }, 0)
            }
            Coordinate(x, y) if y.abs() == 1 => {
                self.position = &self.position + &Coordinate(if x > 0 { x - 1 } else { x + 1 }, y)
            }
            Coordinate(x, y) if x.abs() == 1 => {
                self.position = &self.position + &Coordinate(x, if y > 0 { y - 1 } else { y + 1 })
            }
            _ => unreachable!("Found unmatched vector while following : {:#?}", vec),
        };
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(moves: &Vec<Move>) -> usize {
    let mut head = Head::new();
    let mut positions = HashSet::<Coordinate>::new();

    for mv in moves {
        let new_positions = head.move_with(mv);
        new_positions.into_iter().for_each(|pos| {
            positions.insert(pos);
        });
    }

    positions.len()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test
// }
