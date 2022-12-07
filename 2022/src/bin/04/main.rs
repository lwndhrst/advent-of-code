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
struct Range {
    pub lower: u32,
    pub upper: u32,
}

impl Range {
    pub fn from(range: &str) -> Self {
        let range = range
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Self {
            lower: range[0],
            upper: range[1],
        }
    }

    pub fn contains(&self, other: &Range) -> bool {
        &self.lower <= &other.lower && &self.upper >= &other.upper
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (&self.lower >= &other.lower && &self.lower <= &other.upper)
        || (&self.upper >= &other.lower && &self.upper <= &other.upper)
    }
}

fn main() {
    let path = PathBuf::from("src/bin/04/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let answer = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .map(|v| v.iter().map(|r| Range::from(r)).collect::<Vec<Range>>())
        .filter(|p| p[0].contains(&p[1]) || p[1].contains(&p[0]))
        .count();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let answer = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .map(|v| v.iter().map(|r| Range::from(r)).collect::<Vec<Range>>())
        .filter(|p| p[0].overlaps(&p[1]) || p[1].overlaps(&p[0]))
        .count();

    println!("Part 2: {:?}", answer);
}
