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

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug)]
struct Move {
    pub direction: Direction,
    pub steps: usize,
}

impl Move {
    pub fn from_line(input: &str) -> Self {
        let input = input.split(" ").collect::<Vec<&str>>();

        let direction = match input[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!(),
        };

        let steps = input[1].parse::<usize>().unwrap();

        Self { direction, steps }
    }
}

#[derive(Debug)]
struct Knot {
    pos: (i32, i32),
    tail: Box<Option<Knot>>,
}

impl Knot {
    pub fn add_knot(&mut self, knot: Knot) {
        if let Some(ref mut tail) = *self.tail {
            tail.add_knot(knot);
        } else {
            self.tail = Box::new(Some(knot));
        }
    }

    pub fn move_step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.pos.1 += 1;
            }
            Direction::Right => {
                self.pos.0 += 1;
            }
            Direction::Down => {
                self.pos.1 -= 1;
            }
            Direction::Left => {
                self.pos.0 -= 1;
            }
            Direction::UpLeft => {
                self.pos.1 += 1;
                self.pos.0 -= 1;
            }
            Direction::UpRight => {
                self.pos.1 += 1;
                self.pos.0 += 1;
            }
            Direction::DownLeft => {
                self.pos.1 -= 1;
                self.pos.0 -= 1;
            }
            Direction::DownRight => {
                self.pos.1 -= 1;
                self.pos.0 += 1;
            }
        }
    }

    pub fn compute_step(&mut self, direction: &Direction) {
        self.move_step(direction);

        if let Some(ref mut tail) = *self.tail {
            let h = self.pos;
            let t = tail.pos;
            let d = (h.0 - t.0, h.1 - t.1);

            match d {
                (0, 2) => tail.compute_step(&Direction::Up),
                (0, -2) => tail.compute_step(&Direction::Down),
                (2, 0) => tail.compute_step(&Direction::Right),
                (-2, 0) => tail.compute_step(&Direction::Left),
                (1, 2) => tail.compute_step(&Direction::UpRight),
                (1, -2) => tail.compute_step(&Direction::DownRight),
                (-1, 2) => tail.compute_step(&Direction::UpLeft),
                (-1, -2) => tail.compute_step(&Direction::DownLeft),
                (2, 1) => tail.compute_step(&Direction::UpRight),
                (2, -1) => tail.compute_step(&Direction::DownRight),
                (-2, 1) => tail.compute_step(&Direction::UpLeft),
                (-2, -1) => tail.compute_step(&Direction::DownLeft),
                (2, 2) => tail.compute_step(&Direction::UpRight),
                (2, -2) => tail.compute_step(&Direction::DownRight),
                (-2, 2) => tail.compute_step(&Direction::UpLeft),
                (-2, -2) => tail.compute_step(&Direction::DownLeft),
                _ => {}
            }
        }
    }

    pub fn get_tail_positions(&self) -> Vec<(i32, i32)> {
        let mut positions = vec![self.pos];

        if let Some(tail) = &*self.tail {
            let mut tail_positions = tail.get_tail_positions();
            positions.append(&mut tail_positions);
        }

        positions
    }
}

#[derive(Debug)]
struct Rope {
    head: Knot,
}

impl Rope {
    pub fn new(x: i32, y: i32, n: usize) -> Self {
        let position = (x, y);
        let mut head = Knot {
            pos: position,
            tail: Box::new(None),
        };

        for _ in 0..n {
            head.add_knot(Knot { pos: position, tail: Box::new(None) });
        }

        Self { head }
    }

    pub fn compute_step(&mut self, direction: &Direction) {
        self.head.compute_step(direction);
    }
}

fn main() {
    let path = PathBuf::from("src/bin/09/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let moves = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| Move::from_line(s))
        .collect::<Vec<Move>>();

    let mut visited = HashMap::<(i32, i32), usize>::new();

    let mut rope = Rope::new(255, 255, 1);
    visited.insert((*rope.head.tail).as_ref().unwrap().pos, 1);

    moves.iter().for_each(|m| {
        for _ in 0..m.steps {
            rope.compute_step(&m.direction);
            let count = visited
                .entry((*rope.head.tail).as_ref().unwrap().pos)
                .or_insert(0);
            *count += 1;
        }
    });

    let answer = visited.len();
    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let moves = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| Move::from_line(s))
        .collect::<Vec<Move>>();

    let mut visited = HashMap::<(i32, i32), usize>::new();

    let mut rope = Rope::new(255, 255, 9);
    visited.insert((*rope.head.tail).as_ref().unwrap().pos, 1);

    moves.iter().for_each(|m| {
        for _ in 0..m.steps {
            rope.compute_step(&m.direction);
            let tail = (*rope.head.tail).as_ref().unwrap();
            if let Some(pos) = tail.get_tail_positions().last() {
                let count = visited
                    .entry(*pos)
                    .or_insert(0);
                *count += 1;
            }
        }
    });

    let answer = visited.len();
    println!("Part 2: {:?}", answer);
}
