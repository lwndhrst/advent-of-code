#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod new_line {
    pub const SINGLE: &str = "\r\n";
    pub const DOUBLE: &str = "\r\n\r\n";
}

#[cfg(target_os = "linux")]
mod new_line {
    pub const SINGLE: &str = "\n";
    pub const DOUBLE: &str = "\n\n";
}

// for part 2:
// product of all the prime numbers tested against
const M: usize = 2 * 7 * 3 * 17 * 11 * 19 * 5 * 13;

struct Monkey {
    pub idx: usize,
    pub items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    throw_true: usize,
    throw_false: usize,
    count: usize,
}

impl Monkey {
    pub fn new(
        idx: usize,
        items: Vec<usize>,
        operation: Box<dyn Fn(usize) -> usize>,
        test: Box<dyn Fn(usize) -> bool>,
        throw_true: usize,
        throw_false: usize,
    ) -> Self {
        Self {
            idx,
            items,
            operation,
            test,
            throw_true,
            throw_false,
            count: 0,
        }
    }

    pub fn process_item_part_one(&mut self) -> (usize, usize) {
        self.count += 1;

        let item = self.items.pop().unwrap();
        let worry = (self.operation)(item) / 3;

        if (self.test)(worry) {
            (self.throw_true, worry)
        } else {
            (self.throw_false, worry)
        }
    }

    pub fn process_item_part_two(&mut self) -> (usize, usize) {
        self.count += 1;

        let item = self.items.pop().unwrap();
        let worry = (self.operation)(item) % M;

        if (self.test)(worry) {
            (self.throw_true, worry)
        } else {
            (self.throw_false, worry)
        }
    }
}

fn main() {
    let path = PathBuf::from("src/bin/11/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(_input: &str) {
    let mut monkeys = init_monkeys();
    let mut items: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            if let Some(items) = items.get_mut(&monkey.idx) {
                monkey.items.append(items);
            }

            while monkey.items.len() > 0 {
                let (target, item) = monkey.process_item_part_one();
                let entry = items.entry(target).or_insert(vec![]);
                entry.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.count.cmp(&a.count));
    let rank_one = monkeys[0].count;
    let rank_two = monkeys[1].count;

    let answer = rank_one * rank_two;
    println!("Part 1: {:?}", answer);
}

fn part_two(_input: &str) {
    let mut monkeys = init_monkeys();
    let mut items: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..10_000 {
        for monkey in monkeys.iter_mut() {
            if let Some(items) = items.get_mut(&monkey.idx) {
                monkey.items.append(items);
            }

            while monkey.items.len() > 0 {
                let (target, item) = monkey.process_item_part_two();
                let entry = items.entry(target).or_insert(vec![]);
                entry.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.count.cmp(&a.count));
    let rank_one = monkeys[0].count;
    let rank_two = monkeys[1].count;

    let answer = rank_one * rank_two;
    println!("Part 2: {:?}", answer);
}

fn init_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            0,
            vec![80], 
            Box::new(|old| old * 5),
            Box::new(|a| a & 1 == 0),
            4,
            3,
        ),
        Monkey::new(
            1,
            vec![75, 83, 74], 
            Box::new(|old| old + 7),
            Box::new(|a| a % 7 == 0),
            5,
            6,
        ),
        Monkey::new(
            2,
            vec![86, 67, 61, 96, 52, 63, 73], 
            Box::new(|old| old + 5),
            Box::new(|a| a % 3 == 0),
            7,
            0,
        ),
        Monkey::new(
            3,
            vec![85, 83, 55, 85, 57, 70, 85, 52], 
            Box::new(|old| old + 8),
            Box::new(|a| a % 17 == 0),
            1,
            5,
        ),
        Monkey::new(
            4,
            vec![67, 75, 91, 72, 89], 
            Box::new(|old| old + 4),
            Box::new(|a| a % 11 == 0),
            3,
            1,
        ),
        Monkey::new(
            5,
            vec![66, 64, 68, 92, 68, 77], 
            Box::new(|old| old * 2),
            Box::new(|a| a % 19 == 0),
            6,
            2,
        ),
        Monkey::new(
            6,
            vec![97, 94, 79, 88], 
            Box::new(|old| old * old),
            Box::new(|a| a % 5 == 0),
            2,
            7,
        ),
        Monkey::new(
            7,
            vec![77, 85], 
            Box::new(|old| old + 6),
            Box::new(|a| a % 13 == 0),
            4,
            0,
        ),
    ]
}
