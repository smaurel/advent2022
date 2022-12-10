use std::{
    collections::{HashMap, HashSet},
    iter::Rev,
    ops::Range,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Either;

#[aoc_generator(day8)]
pub fn parse(
    input: &str,
) -> (
    HashMap<(usize, usize), u32>,
    (usize, usize),
    HashMap<u32, Vec<(usize, usize)>>,
) {
    let mut sizes = (0, 0);
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();
    let mut trees_by_size: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(col_index, c)| {
            if line_index + 1 > sizes.0 {
                sizes.0 = line_index + 1;
            }
            if col_index + 1 > sizes.1 {
                sizes.1 = line_index + 1;
            }
            let tree_height = c.to_digit(10).expect("cannot parse");
            let trees_of_height = trees_by_size.entry(tree_height).or_insert(vec![]);
            trees_of_height.push((col_index, line_index));
            grid.insert((col_index, line_index), tree_height);
        })
    });
    (grid, sizes, trees_by_size)
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

pub fn get_distance_from(base_tree: &(i32, i32), other_tree: &(i32, i32)) -> (usize, usize) {
    match (base_tree.0 - other_tree.0, base_tree.1 - other_tree.1) {
        (col_distance, line_distance) if col_distance > 0 && line_distance == 0 => {
            (col_distance as usize, 3)
        }
        (col_distance, line_distance) if col_distance < 0 && line_distance == 0 => {
            (-col_distance as usize, 1)
        }
        (col_distance, line_distance) if col_distance == 0 && line_distance > 0 => {
            (line_distance as usize, 0)
        }
        (col_distance, line_distance) if col_distance == 0 && line_distance < 0 => {
            (-line_distance as usize, 2)
        }
        _ => unreachable!("comapred to self ??"),
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(
    (grid, (col_size, line_size), _): &(
        HashMap<(usize, usize), u32>,
        (usize, usize),
        HashMap<u32, Vec<(usize, usize)>>,
    ),
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

#[aoc(day8, part2)]
pub fn solve_part2(
    (grid, (col_size, line_size), trees_by_size): &(
        HashMap<(usize, usize), u32>,
        (usize, usize),
        HashMap<u32, Vec<(usize, usize)>>,
    ),
) -> usize {
    let mut max_score = 0;
    for tested_height in (5..10).rev() {
        let default_vec: Vec<(usize, usize)> = vec![];
        let trees_of_size = match trees_by_size.get(&tested_height) {
            Some(vector) => vector,
            None => &default_vec,
        };
        // Top Right Bottom Left
        for tree in trees_of_size {
            if tree.0 == 0 || tree.1 == 0 || tree.0 == line_size - 1 || tree.1 == col_size - 1 {
                continue;
            }

            // set distance to closest by using forest boundaries as a default value
            let mut distance_to_closest: Vec<(usize, usize)> = vec![
                (tree.1, *(grid.get(&(tree.0, 0)).unwrap()) as usize),
                (
                    line_size - 1 - tree.0,
                    *(grid.get(&(tree.1, line_size - 1)).unwrap()) as usize,
                ),
                (
                    col_size - 1 - tree.1,
                    *(grid.get(&(col_size - 1, tree.0)).unwrap()) as usize,
                ),
                (tree.0, *(grid.get(&(tree.1, 0)).unwrap()) as usize),
            ];
            for other_height in tested_height..10 {
                let trees_of_other_size = match trees_by_size.get(&other_height) {
                    Some(vector) => vector,
                    None => &default_vec,
                };
                for other_tree in trees_of_other_size {
                    if (other_tree.0 != tree.0 && other_tree.1 != tree.1) || other_tree == tree {
                        continue;
                    }

                    let (dist, index) = get_distance_from(
                        &(tree.0 as i32, tree.1 as i32),
                        &(other_tree.0 as i32, other_tree.1 as i32),
                    );

                    let closest = distance_to_closest[index];
                    if closest.0 > dist {
                        distance_to_closest[index] = (dist, other_height as usize);
                    }
                }
            }
            let score: usize = distance_to_closest.iter().map(|dist| dist.0).product();
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}
