use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn parse(input: &str) -> (HashMap<(usize, usize), u32>, (usize, usize)) {
    let mut sizes = (0, 0);
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(col_index, c)| {
            if line_index + 1 > sizes.0 {
                sizes.0 = line_index + 1;
            }
            if col_index + 1 > sizes.1 {
                sizes.1 = line_index + 1;
            }
            grid.insert(
                (col_index, line_index),
                c.to_digit(10).expect("cannot parse"),
            );
        })
    });
    (grid, sizes)
}

#[aoc(day8, part1)]
pub fn solve_part1(
    (grid, (line_size, col_size)): &(HashMap<(usize, usize), u32>, (usize, usize)),
) -> usize {
    5
}
