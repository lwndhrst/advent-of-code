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
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    pub fn from_line(input: &str) -> Self {
        let parts = input
            .split(" ")
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();

        if parts.len() == 1 {
            Instruction::Noop
        } else {
            Instruction::Add(parts[1].parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct CPU {
    pub counter: usize,
    pub x: i32,
    crt: CRT,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            counter: 0,
            x: 1,
            crt: CRT {
                lines: Vec::with_capacity(6),
            },
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Option<i32> {
        let mut result = None;

        match instruction {
            Instruction::Noop => {
                self.crt.render(self.counter, self.x);
                self.counter += 1;
                if self.is_answer_val(self.counter) {
                    result = Some(self.signal_strength());
                }
            }
            Instruction::Add(y) => {
                self.crt.render(self.counter, self.x);
                self.counter += 1;
                if self.is_answer_val(self.counter) {
                    result = Some(self.signal_strength());
                }
                self.crt.render(self.counter, self.x);
                self.counter += 1;
                if self.is_answer_val(self.counter) {
                    result = Some(self.signal_strength());
                }
                self.x += y;
            }
        }

        result
    }

    fn is_answer_val(&self, count: usize) -> bool {
        if count == 20 {
            return true;
        }

        if count < 60 {
            return false;
        }

        return self.is_answer_val(count - 40);
    }

    fn signal_strength(&self) -> i32 {
        self.counter as i32 * self.x
    }
}

#[derive(Debug)]
struct CRT {
    lines: Vec<String>,
}

impl CRT {
    pub fn render(&mut self, counter: usize, x: i32) {
        let u = (counter % 40) as i32;
        let v = counter / 40;

        if self.lines.len() < v + 1 {
            self.lines.push(String::with_capacity(40));
        }

        if u < x - 1 || u > x + 1 {
            self.lines[v].push('.');
        } else {
            self.lines[v].push('#');
        }
    }

    pub fn print(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }
}

fn main() {
    let path = PathBuf::from("src/bin/10/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let instructions = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| Instruction::from_line(s))
        .collect::<Vec<Instruction>>();

    let mut cpu = CPU::new();
    let mut answer = vec![];

    for instruction in instructions {
        if let Some(x) = cpu.execute(&instruction) {
            answer.push(x);
        }
    }

    let answer = answer.iter().sum::<i32>();
    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let instructions = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| Instruction::from_line(s))
        .collect::<Vec<Instruction>>();

    let mut cpu = CPU::new();
    let mut answer = vec![];

    for instruction in instructions {
        if let Some(x) = cpu.execute(&instruction) {
            answer.push(x);
        }
    }

    println!("Part 2:");
    cpu.crt.print();
}
