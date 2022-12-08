use std::{collections::HashSet, hash::Hash, rc::Rc};

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, Hash)]
pub struct Folder {
    size: usize,
    path: Rc<Path>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Path {
    name: String,
    // Represents a stack of directory name, up to /
    previous_directory: Option<Rc<Path>>,
}

pub struct BrowsingState {
    folder_stack: Vec<Folder>,
    other_folders: HashSet<Folder>,
}

impl BrowsingState {
    pub fn cd_dir(&mut self, line: &str) {
        let dir_name = line.split(' ').last().expect("dir");
        match dir_name {
            ".." => self.pop_folder(),
            _ => self.push_on_stack(dir_name),
        }
    }

    pub fn pop_folder(&mut self) {
        let popped_dir = self.folder_stack.pop().expect("pop");
        let new_current_folder = self.folder_stack.last_mut();
        match new_current_folder {
            Some(f) => f.size += popped_dir.size,
            None => {}
        };
        self.other_folders.insert(popped_dir);
    }

    pub fn push_on_stack(&mut self, dir_name: &str) {
        let new_path = match self.folder_stack.last() {
            Some(folder) => Path {
                name: folder.path.name.to_string() + "/" + dir_name,
                previous_directory: Some(Rc::clone(&folder.path)),
            },
            None => Path {
                name: dir_name.to_owned(),
                previous_directory: None,
            },
        };
        let new_folder = Folder {
            size: 0,
            path: Rc::new(new_path),
        };
        self.folder_stack.push(new_folder);
    }

    pub fn add_size(&mut self, line: &str) {
        let size: usize = line
            .split(' ')
            .next()
            .expect("line empty")
            .parse()
            .expect(line);
        let curr_folder = self.folder_stack.last_mut().expect("curr_folder");
        curr_folder.size += size;
    }

    pub fn back_to_dir(&mut self) {
        while self.folder_stack.len() > 0 {
            self.pop_folder()
        }
    }

    pub fn get_under_size(&self, size: usize) -> usize {
        self.other_folders
            .iter()
            .filter(|f| f.size <= size)
            .map(|f| f.size)
            .sum()
    }

    pub fn get_total_size(&self) -> usize {
        self.other_folders
            .iter()
            .find(|f| f.path.name == "/")
            .expect("could not find root dir")
            .size
    }

    pub fn find_smallest(&self, size_to_free: usize) -> &Folder {
        self.other_folders
            .iter()
            .filter(|f| f.size >= size_to_free)
            .min_by_key(|f| f.size)
            .expect("could not find min")
    }
}

pub fn parse_folders(input: &str) -> BrowsingState {
    let folder_stack: Vec<Folder> = vec![];
    let other_folders: HashSet<Folder> = HashSet::new();
    let mut browsing_state = BrowsingState {
        folder_stack,
        other_folders,
    };
    for line in input.lines() {
        match &line[..4] {
            // "push or pop on stack"
            "$ cd" => browsing_state.cd_dir(line),
            "$ ls" => {}
            "dir " => {}
            _ => browsing_state.add_size(line),
        };
    }

    browsing_state.back_to_dir();
    browsing_state
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let browsing_state = parse_folders(input);
    browsing_state.get_under_size(100000)
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let browsing_state = parse_folders(input);
    let size_to_free = browsing_state.get_total_size() - 40000000;
    browsing_state.find_smallest(size_to_free).size
}
