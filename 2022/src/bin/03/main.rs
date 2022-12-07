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

fn main() {
    let path = PathBuf::from("src/bin/03/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let answer = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(split_compartments)
        .map(find_duplicate)
        .map(get_value)
        .sum::<u32>();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let backpacks = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .collect::<Vec<&str>>();

    let mut answer = Vec::new();
    let mut offset = 0;
    loop {
        if offset > backpacks.len() - 3 {
            break;
        }

        let g = &backpacks[offset..(offset + 3)];
        let c = g[0].chars().find(|c| g[1].contains(*c) && g[2].contains(*c)).unwrap();
        answer.push(get_value(c));

        offset += 3;
    }

    let answer = answer.iter().sum::<u32>();

    println!("Part 2: {:?}", answer);
}

fn split_compartments(s: &str) -> Vec<&str> {
    let idx = s.len() / 2;
    vec![&s[..idx], &s[idx..]]
}

fn find_duplicate(b: Vec<&str>) -> char {
    b[0].chars().find(|c| b[1].contains(*c)).unwrap()
}

fn get_value(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - (64 - 26)
    }
}
