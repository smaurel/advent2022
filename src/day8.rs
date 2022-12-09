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
            let mut trees_of_height = trees_by_size.entry(tree_height).or_insert(vec![]);
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
            (col_distance as usize, 1)
        }
        (col_distance, line_distance) if col_distance < 0 && line_distance == 0 => {
            (-col_distance as usize, 3)
        }
        (col_distance, line_distance) if col_distance == 0 && line_distance > 0 => {
            (line_distance as usize, 2)
        }
        (col_distance, line_distance) if col_distance == 0 && line_distance < 0 => {
            (-line_distance as usize, 0)
        }
        _ => unreachable!("comapred to self ??"),
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(
    (grid, (line_size, col_size), _): &(
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
    (_, _, trees_by_size): &(
        HashMap<(usize, usize), u32>,
        (usize, usize),
        HashMap<u32, Vec<(usize, usize)>>,
    ),
) -> usize {
    let mut max_score = 0;
    for tested_height in (7..10).rev() {
        // Top Right Bottom Left
        let mut distance_to_closest: Vec<usize> = vec![];
        for tree in trees_by_size
            .get(&tested_height)
            .expect("getting wrong height")
        {
            // set distance to closest by using forest boundaries as a default value
            for other_height in tested_height..10 {
                for other_tree in trees_by_size
                    .get(&other_height)
                    .expect("getting wrong height")
                {
                    if other_tree.0 != tree.0 && other_tree.1 != tree.1 || other_tree == tree {
                        continue;
                    }

                    let (dist, index) = get_distance_from(
                        &(tree.0 as i32, tree.1 as i32),
                        &(other_tree.0 as i32, other_tree.1 as i32),
                    );
                    if distance_to_closest[index] > dist {
                        distance_to_closest[index] = dist;
                    }
                }
            }
            let score: usize = distance_to_closest.iter().product();
            if score > max_score {
                println!(
                    "setting new max for tree of coords {:#?}, with distances  {:#?}",
                    &tree, &distance_to_closest
                );
                max_score = score;
            }
        }
    }
    max_score
}
