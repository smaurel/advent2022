use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, Sub},
};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Coordinate(i32, i32);

impl Coordinate {
    pub fn is_adjacent_with(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= 1 && (self.1 - other.1).abs() <= 1
    }

    pub fn distance(&self, other: &Self) -> usize {
        let dist_vec = other - self;
        (dist_vec.0.abs() + dist_vec.1.abs()) as usize
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

pub struct Pathfinding {
    height_map: HashMap<Coordinate, usize>,
    current_frontier: HashSet<Coordinate>,
    predecessor: HashMap<Coordinate, Coordinate>,
    point_scores: HashMap<Coordinate, usize>,
    goal_point: Coordinate,
}

const DIRECTIONS: &[Coordinate] = &[
    Coordinate(-1, 0),
    Coordinate(0, -1),
    Coordinate(1, 0),
    Coordinate(0, 1),
];

impl Pathfinding {
    fn new(
        height_map: HashMap<Coordinate, usize>,
        starting_point: Coordinate,
        goal_point: Coordinate,
    ) -> Self {
        let mut frontier = HashSet::new();
        frontier.insert(starting_point);
        let mut point_scores = HashMap::new();
        point_scores.insert(starting_point, 1);
        Self {
            current_frontier: frontier,
            height_map: height_map,
            goal_point: goal_point,
            point_scores: point_scores,
            predecessor: HashMap::new(),
        }
    }

    pub fn neighbours(&self, point: &Coordinate) -> Vec<Coordinate> {
        let height = self.get_height(point);

        DIRECTIONS
            .iter()
            .map(|dir| point + dir)
            .filter(|p| self.valid(p) && self.get_height(p) <= height + 1)
            .collect_vec()
    }

    pub fn valid(&self, point: &Coordinate) -> bool {
        self.height_map.contains_key(point)
    }

    pub fn get_height(&self, point: &Coordinate) -> usize {
        *self.height_map.get(point).expect("oob getting height")
    }

    pub fn update_frontier_with(&mut self, coming_from: &Coordinate, steps: usize) {
        for neighbour in self.neighbours(coming_from) {
            if *self.point_scores.get(&neighbour).unwrap_or(&usize::MAX) > steps + 1 {
                self.point_scores.insert(neighbour, steps + 1);
                self.predecessor.insert(neighbour, *coming_from);
                if !self.current_frontier.contains(&neighbour) {
                    self.current_frontier.insert(neighbour);
                }
            }
        }
    }

    pub fn get_best_candidate(&mut self) -> Coordinate {
        let mut closest_point = Coordinate(-1, -1);
        for candidate in &self.current_frontier {
            if closest_point == Coordinate(-1, -1) {
                closest_point = *candidate;
            }
            if *self.point_scores.get(&candidate).unwrap() + candidate.distance(&self.goal_point)
                < *self.point_scores.get(&closest_point).unwrap()
                    + closest_point.distance(&self.goal_point)
            {
                closest_point = *candidate;
            }
        }

        self.current_frontier.remove(&closest_point);
        closest_point
    }

    pub fn search_for_closest_path(&mut self) {
        loop {
            let best_candidate = self.get_best_candidate();
            if best_candidate == self.goal_point {
                return;
            }
            self.update_frontier_with(
                &best_candidate,
                *self.point_scores.get(&best_candidate).unwrap(),
            );
        }
    }

    pub fn collect_path(&self) -> Vec<Coordinate> {
        let mut res: Vec<Coordinate> = vec![self.goal_point];
        let mut curr_point = self.goal_point;
        while let Some(point) = self.predecessor.get(&curr_point) {
            res.push(*point);
            curr_point = *point;
        }
        res
    }
}

pub fn parse(input: &str) -> (HashMap<Coordinate, usize>, Coordinate, Coordinate) {
    let mut height_grid: HashMap<Coordinate, usize> = HashMap::new();
    let mut starting_point = Coordinate(-1, -1);
    let mut goal_point = Coordinate(-1, -1);
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    starting_point = Coordinate(i as i32, j as i32);
                    height_grid.insert(Coordinate(i as i32, j as i32), 'a' as usize);
                }
                'E' => {
                    goal_point = Coordinate(i as i32, j as i32);
                    height_grid.insert(Coordinate(i as i32, j as i32), 'z' as usize);
                }
                _ => {
                    height_grid.insert(Coordinate(i as i32, j as i32), c as usize);
                }
            };
        }
    }
    (height_grid, starting_point, goal_point)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (height_map, starting_point, goal_point) = parse(input);
    let mut pathfinding = Pathfinding::new(height_map, starting_point, goal_point);
    pathfinding.search_for_closest_path();
    pathfinding.collect_path().len() - 1
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (height_map, _, goal_point) = parse(input);
    let mut pathfinding = Pathfinding::new(height_map, Coordinate(0, 33), goal_point);
    pathfinding.search_for_closest_path();
    pathfinding.collect_path().len() - 1
}
