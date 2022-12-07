#![allow(dead_code)]

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

#[derive(Debug)]
struct Crane {
    pub stacks: Vec<Vec<String>>
}

impl Crane {
    pub fn from_start_config(config: &str) -> Self {
        let config = config
            .split(new_line::SINGLE)
            .map(|s| s.replace("    ", " ---"))
            .filter(|s| !s.starts_with(" "))
            .map(|s| s.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>())
            .collect::<Vec<_>>();

        let mut stacks: Vec<Vec<String>> = vec![];
        for _ in 0..9 {
            stacks.push(vec![]);
        }
        
        for line in config.iter().rev() {
            for i in 0..line.len() {
                let item = &line[i];
                if item == "---" {
                    continue;
                }

                stacks[i].push(item.to_owned());
            }
        }

        Self { stacks }
    }

    pub fn part_one(&mut self, proc: &Vec<Instruction>) {
        for i in proc {
            let mut remaining = i.amount;
            while remaining > 0 {
                let item = self.stacks[i.from].pop().unwrap();
                self.stacks[i.to].push(item);
                remaining -= 1;
            }
        }
    }

    pub fn part_two(&mut self, proc: &Vec<Instruction>) {
        for i in proc {
            let mut buf = vec![];
            let mut remaining = i.amount;
            while remaining > 0 {
                let item = self.stacks[i.from].pop().unwrap();
                buf.push(item);
                remaining -= 1;
            }
            buf.reverse();
            self.stacks[i.to].append(&mut buf);
        }
    }

    pub fn tops(&self) -> Vec<&String> {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    pub fn from_line(l: &str) -> Self {
        let v = l
            .split(" ")
            .filter(|s| s.parse::<usize>().is_ok())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self { amount: v[0], from: v[1] - 1, to: v[2] - 1 }
    }
}

fn main() {
    let path = PathBuf::from("src/bin/05/input.txt");
    let input = read_to_string(path).unwrap();

    run(&input);
}

fn run(input: &str) {
    let input = input.split(new_line::DOUBLE).collect::<Vec<&str>>();

    let start_config = input[0];
    let mut crane = Crane::from_start_config(start_config);
    let procedure = input[1]
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(Instruction::from_line)
        .collect::<Vec<Instruction>>();

    crane.part_one(&procedure);
    println!("Part 1: {:?}", crane.tops());

    let mut crane = Crane::from_start_config(start_config);
    crane.part_two(&procedure);
    println!("Part 2: {:?}", crane.tops());
}
