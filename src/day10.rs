use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut split_input = input.split_whitespace();
        match split_input
            .next()
            .expect(format!("nothing in line ? {input}").as_str())
        {
            "addx" => Instruction::Addx(
                split_input
                    .next()
                    .expect(format!("nothing after addx for line {}", input).as_str())
                    .parse()
                    .expect("could not parse"),
            ),
            "noop" => Instruction::Noop,
            _ => unreachable!("Received unrecognized operation"),
        }
    }
}

pub struct Registry<'a> {
    cycle_count: usize,
    value: i32,
    signal_strength: i32,
    desired_cycles: &'a [usize],
    current_crt: String,
    crt_output: Vec<String>,
    desired_cycle_index: usize,
}

impl Registry<'_> {
    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.cycle_count += 1;
                self.update_signal_strength();
            }
            Instruction::Addx(v) => {
                self.cycle_count += 1;
                self.update_signal_strength();
                self.cycle_count += 1;
                self.update_signal_strength();
                self.value += v;
            }
        };
    }

    pub fn update_signal_strength(&mut self) {
        if self.desired_cycle_index >= self.desired_cycles.len() {
            return;
        }

        if ((self.cycle_count % 40) as i32 - 1 - self.value).abs() <= 1 {
            self.current_crt.push_str("#");
        } else {
            self.current_crt.push_str(".")
        }

        if self.desired_cycles[self.desired_cycle_index] <= self.cycle_count {
            self.signal_strength +=
                (self.desired_cycles[self.desired_cycle_index] as i32) * self.value;
            self.desired_cycle_index += 1;
            self.crt_output.push(self.current_crt.clone());
            self.current_crt.clear()
        }
    }
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Instruction>>()
}

#[aoc(day10, part1)]
pub fn solve_part1(instructions: &Vec<Instruction>) -> i32 {
    let mut registry = Registry {
        cycle_count: 0,
        value: 1,
        signal_strength: 0,
        current_crt: String::from(""),
        crt_output: vec![],
        desired_cycles: &[20, 60, 100, 140, 180, 220],
        desired_cycle_index: 0,
    };

    for instruction in instructions {
        registry.execute_instruction(instruction);
    }

    registry.signal_strength
}

#[aoc(day10, part2)]
pub fn solve_part2(instructions: &Vec<Instruction>) -> i32 {
    let mut registry = Registry {
        cycle_count: 0,
        value: 1,
        signal_strength: 0,
        current_crt: String::from(""),
        crt_output: vec![],
        desired_cycles: &[40, 80, 120, 160, 200, 240],
        desired_cycle_index: 0,
    };

    for instruction in instructions {
        registry.execute_instruction(instruction);
    }

    for crt_line in registry.crt_output {
        println!("{}", crt_line);
    }

    registry.signal_strength
}
