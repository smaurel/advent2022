use std::{collections::HashSet, hash::Hash};

use aoc_runner_derive::aoc;

pub struct Folder {
    size: usize,
    name: String,
    path: Path,
}

pub struct Path {
    path: String,
    // Represents a stack of directory name, up to /
    previous_directories: Vec<String>,
}

pub fn cd_dir(folder_stack: &mut Vec<Folder>, all_folders: &mut HashSet<&Folder>, line: &str) {
    let dir_name = line.split(' ').last().unwrap();
    match dir_name {
        ".." => println!("pop_stack"),
        _ => push_on_stack(folder_stack, all_folders, line),
    }
}

pub fn push_on_stack(
    folder_stack: &mut Vec<Folder>,
    all_folders: &mut HashSet<&Folder>,
    dir_name: &str,
) {
    let previous_path = folder_stack.last();
    let new_path = match previous_path {
        Some(folder) => Path {
            path: String::from(folder.path.path + "/" + dir_name),
            previous_directories: folder
                .path
                .previous_directories
                .dedup(String::from(dir_name)),
        },
        None => Path {
            path: String::from(dir_name),
            previous_directories: vec![],
        },
    };
    let new_folder = Folder {
        size: 0,
        name: String::from(dir_name),
        path: previous_path,
    };
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut folder_stack: Vec<Folder> = vec![];
    let mut all_folders: HashSet<&Folder> = HashSet::new();
    for line in input.lines() {
        match &line[..5] {
            // "push or pop on stack"
            "$ cd" => push_on_stack(&mut folder_stack, &mut all_folders, line),
            "$ ls" => println!("doing ls, do nothing"),
            "dir " => println!("directory in ls, do nothing"),
            _ => println!("add file size to current dire"),
        }
    }
    5
    //     input
    //         .lines()
    //         .map(|line| get_first_marker(line, 4))
    //         .next()
    //         .unwrap()
}

// #[aoc(day6, part2)]
// pub fn solve_part2(input: &str) -> usize {
//     input
//         .lines()
//         .map(|line| get_first_marker(line, 14))
//         .next()
//         .unwrap()
// }
