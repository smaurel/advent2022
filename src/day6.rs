use aoc_runner_derive::aoc;

pub fn get_first_marker(line: &str) -> usize {
    for i in 0..line.len() - 3 {
        let curr_range = &line[i..i + 4];
        if curr_range
            .chars()
            .map(|c| curr_range.chars().filter(|ch| *ch == c).count())
            .sum::<usize>()
            == curr_range.len()
        {
            return i + 4;
        }
    }
    0
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(|line| get_first_marker(line)).sum()
}

// #[aoc(day6, part2)]
// pub fn solve_part2(input: &str) -> String {
// let (mut stacks, moves) = parse_input_generator(input);
// moves.iter().for_each(|mv| mv.exec_p2(&mut stacks));
// stacks.iter().map(|stack| stack.last().unwrap()).collect()
// }
