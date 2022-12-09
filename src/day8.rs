use std::{
    collections::{HashMap, HashSet},
    iter::Rev,
    ops::Range,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Either;

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

pub fn get_range(max: &usize, reverse: bool) -> Either<Range<usize>, Rev<Range<usize>>> {
    let range = 0..*max;
    if reverse {
        Either::Right(range.rev())
    } else {
        Either::Left(range)
    }
}

pub fn browse_column(
    line_index: usize,
    column_size: &usize,
    grid: &HashMap<(usize, usize), u32>,
    reverse: bool,
    mut visible_trees: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut highest_for_col = -1;
    for col_index in get_range(column_size, reverse) {
        let curr_height = grid
            .get(&(col_index, line_index))
            .expect("tried to get outside of grid");
        if *curr_height as i32 > highest_for_col {
            highest_for_col = *curr_height as i32;
            visible_trees.insert((col_index, line_index));
        }
        if curr_height == &9 {
            break;
        }
    }
    visible_trees
}

pub fn browse_line(
    col_index: usize,
    line_size: &usize,
    grid: &HashMap<(usize, usize), u32>,
    reverse: bool,
    mut visible_trees: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut highest_for_line = -1;
    for line_index in get_range(line_size, reverse) {
        let curr_height = grid
            .get(&(col_index, line_index))
            .expect("tried to get outside of grid");
        if *curr_height as i32 > highest_for_line {
            highest_for_line = *curr_height as i32;
            visible_trees.insert((col_index, line_index));
        }
        if curr_height == &9 {
            break;
        }
    }
    visible_trees
}

#[aoc(day8, part1)]
pub fn solve_part1(
    (grid, (line_size, col_size)): &(HashMap<(usize, usize), u32>, (usize, usize)),
) -> usize {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    // Browse by column
    for i in 0..*line_size {
        visible_trees = browse_column(i, &col_size, &grid, false, visible_trees);
        visible_trees = browse_column(i, &col_size, &grid, true, visible_trees);
    }

    for i in 0..*col_size {
        visible_trees = browse_line(i, &line_size, &grid, false, visible_trees);
        visible_trees = browse_line(i, &line_size, &grid, true, visible_trees);
    }

    visible_trees.iter().count()
}
