use std::sync::atomic::{AtomicU64, Ordering};
use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::aoc;

static PPCM: AtomicU64 = AtomicU64::new(1);

pub fn parse_monkeys(input: &str) -> Vec<Rc<RefCell<Monkey>>> {
    PPCM.store(1, Ordering::Relaxed);
    let monkeys = input
        .trim()
        .split("\n\n")
        .map(|line| line.into())
        .map(|monkey| Rc::new(RefCell::new(monkey)))
        .collect::<Vec<Rc<RefCell<Monkey>>>>();

    // Optimisation :  in order to get values in check, we need to compute ppcm of all dividers
    for monkey in &monkeys {
        PPCM.store(
            num::integer::lcm(
                monkey.borrow().test.divisibility as u64,
                PPCM.load(Ordering::Relaxed),
            ),
            Ordering::Relaxed,
        );
    }

    monkeys
}

pub fn parse_last_num(input: &str) -> usize {
    input
        .split_whitespace()
        .last()
        .expect("last in parse last")
        .parse::<usize>()
        .expect("parse in parse last")
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.trim().lines();
        // On monkey identifier
        lines.next();
        let starting_items: Vec<u64> = lines
            .next()
            .expect("could not find next for starting items")
            .trim()
            .strip_prefix("Starting items: ")
            .expect("could not strip prefix for items")
            .split(", ")
            .map(|item| item.parse::<u64>().expect("parsing item"))
            .collect();
        let operation_elements = lines
            .next()
            .expect("next for op")
            .trim()
            .strip_prefix("Operation: new = ")
            .expect("strip op")
            .split_whitespace()
            .collect::<Vec<&str>>();
        let operation = Operation {
            value1: operation_elements[0].into(),
            op: operation_elements[1].into(),
            value2: operation_elements[2].into(),
        };

        let test = Test {
            divisibility: parse_last_num(lines.next().unwrap()),
            if_true: parse_last_num(lines.next().unwrap()),
            if_false: parse_last_num(lines.next().unwrap()),
        };
        Self {
            items: starting_items,
            operation,
            test,
            inspected_items: 0,
        }
    }
}

pub struct Operation {
    op: Operand,
    value1: OperationValue,
    value2: OperationValue,
}

impl Operation {
    pub fn run(&self, input_value: u64) -> u64 {
        let v1 = match self.value1 {
            OperationValue::ConstantValue(x) => x as u64,
            OperationValue::InputValue => input_value,
        };
        let v2 = match self.value2 {
            OperationValue::ConstantValue(x) => x as u64,
            OperationValue::InputValue => input_value,
        };

        let result = match self.op {
            Operand::Add => v1 + v2,
            Operand::Mult => v1 * v2,
        };

        // Taking this value here does not affect divisibility by any of values from which
        // LCM has been computed, and helps us keeping values from overflowing
        result % PPCM.load(Ordering::Relaxed)
    }
}

pub enum Operand {
    Mult,
    Add,
}

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input {
            "*" => Operand::Mult,
            "+" => Operand::Add,
            _ => unreachable!("unknown operand {}", input),
        }
    }
}

pub enum OperationValue {
    ConstantValue(usize),
    InputValue,
}

impl From<&str> for OperationValue {
    fn from(input: &str) -> Self {
        match input {
            "old" => Self::InputValue,
            x => Self::ConstantValue(x.parse::<usize>().expect("parse op")),
        }
    }
}

pub struct Test {
    divisibility: usize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    pub fn monkey_to_throw_to(&self, worry_level: &u64) -> usize {
        if worry_level % self.divisibility as u64 == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspected_items: usize,
}

impl Monkey {
    pub fn take_turn(&mut self, all_monkeys: &Vec<Rc<RefCell<Monkey>>>, with_division: bool) {
        if self.items.len() == 0 {
            return;
        }

        for item in &self.items {
            let mut i = item.clone();
            i = self.operation.run(i);
            if with_division {
                i = i / 3;
            }
            let monkey_to_throw = self.test.monkey_to_throw_to(&i);
            all_monkeys[monkey_to_throw as usize]
                .borrow_mut()
                .items
                .push(i);
        }
        self.inspected_items += self.items.len();
        self.items = vec![];
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    {
        for i in 0..20 {
            for monkey in &monkeys {
                let mut monkey = monkey.borrow_mut();
                monkey.take_turn(&monkeys, true);
            }
        }
    }

    let (mut max1, mut max2): (usize, usize) = (0, 0);
    for monkey in &monkeys {
        let monkey = monkey.borrow();
        let inspected_items = monkey.inspected_items;
        if inspected_items > max1 && max1 <= max2 {
            max1 = inspected_items;
            continue;
        }
        if inspected_items > max2 {
            max2 = inspected_items;
            continue;
        }
    }
    max1 * max2
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    {
        for _ in 0..10000 {
            for monkey in &monkeys {
                monkey.borrow_mut().take_turn(&monkeys, false);
            }
        }
    }

    let (mut max1, mut max2): (usize, usize) = (0, 0);
    for monkey in &monkeys {
        let monkey = monkey.borrow();
        let inspected_items = monkey.inspected_items;
        if inspected_items > max1 && max1 <= max2 {
            max1 = inspected_items;
            continue;
        }
        if inspected_items > max2 {
            max2 = inspected_items;
            continue;
        }
    }
    max1 * max2
}
