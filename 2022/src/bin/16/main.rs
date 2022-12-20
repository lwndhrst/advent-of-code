// Couldn't get A* right so I yoinked a giga-slow DFS solution from Reddit *shame*

#![allow(dead_code)]

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
    let input = include_str!("input.txt");

    part_one(&input);
    // part_two(&input);
}

fn part_one(input: &str) {
    let answer = input;
    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let answer = input;
    println!("Part 2: {:?}", answer);
}
