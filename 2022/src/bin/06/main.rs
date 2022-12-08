#![allow(dead_code)]

use std::collections::VecDeque;
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
    let path = PathBuf::from("src/bin/06/input.txt");
    let input = read_to_string(path).unwrap().replace(new_line::SINGLE, "");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut buf = VecDeque::new();

    let mut counter = 0;
    for c in input.chars() {
        counter += 1;
        buf.push_front(c);

        if buf.len() > 4 {
            buf.pop_back();
        }

        if buf.len() == 4 && !has_duplicates(buf.clone()) {
            break;
        }
    }


    let answer = counter;
    println!("Part 1: {:?}", answer);
    println!("Part 1: {:?}", buf);
}

fn part_two(input: &str) {
    let mut buf = VecDeque::new();

    let mut counter = 0;
    for c in input.chars() {
        counter += 1;
        buf.push_front(c);

        if buf.len() > 14 {
            buf.pop_back();
        }

        if buf.len() == 14 && !has_duplicates(buf.clone()) {
            break;
        }
    }


    let answer = counter;
    println!("Part 2: {:?}", answer);
    println!("Part 2: {:?}", buf);
}

fn has_duplicates(mut buf: VecDeque<char>) -> bool {
    while buf.len() > 0 {
        let c = buf.pop_back().unwrap();
        if buf.contains(&c) {
            return true;
        }
    }

    false
}
