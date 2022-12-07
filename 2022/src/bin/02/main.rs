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

const A: u32 = 1;
const B: u32 = 2;
const C: u32 = 3;

const X: u32 = 1;
const Y: u32 = 2;
const Z: u32 = 3;

const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn main() {
    let path = PathBuf::from("src/bin/02/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let answer = input
        .split(new_line::SINGLE)
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().map(|c| get_points(c)).collect::<Vec<u32>>())
        .map(|v| calc_points(v[0], v[1]))
        .sum::<u32>();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let answer = input
        .split(new_line::SINGLE)
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().map(|c| get_points(c)).collect::<Vec<u32>>())
        .map(|v| vec![v[0], choose(v[0], v[1])])
        .map(|v| calc_points(v[0], v[1]))
        .sum::<u32>();

    println!("Part 2: {:?}", answer);
}

fn choose(opponent: u32, code: u32) -> u32 {
    match (opponent, code) {
        (A, X) => Z,
        (B, X) => X,
        (C, X) => Y,
        (A, Z) => Y,
        (B, Z) => Z,
        (C, Z) => X,
        _ => opponent,
    }
}

fn get_points(c: &str) -> u32 {
    match c {
        "A" => A,
        "B" => B,
        "C" => C,
        "X" => X,
        "Y" => Y,
        "Z" => Z,
        _ => 0,
    }
}

fn calc_points(opponent: u32, player: u32) -> u32 {
    player + match (opponent, player) {
        (A, Y) => WIN,
        (B, Z) => WIN,
        (C, X) => WIN,
        (A, Z) => LOSS,
        (B, X) => LOSS,
        (C, Y) => LOSS,
        _ => DRAW,
    }
}
