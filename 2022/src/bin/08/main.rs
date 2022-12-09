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
    let path = PathBuf::from("src/bin/08/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let grid = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut answer = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x];

            let row = &grid[y];
            let visible_row =
                is_visible(&row[0..x], height) || is_visible(&row[(x + 1)..row.len()], height);

            let col = grid.iter().map(|r| r[x]).collect::<Vec<u32>>();
            let visible_col =
                is_visible(&col[0..y], height) || is_visible(&col[(y + 1)..col.len()], height);

            if visible_row || visible_col {
                answer += 1;
            }
        }
    }

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let grid = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut answer = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x];

            let row = &grid[y];
            let left = view_distance(&row[0..x], height, true);
            let right = view_distance(&row[(x + 1)..row.len()], height, false);

            let col = grid.iter().map(|r| r[x]).collect::<Vec<u32>>();
            let up = view_distance(&col[0..y], height, true);
            let down = view_distance(&col[(y + 1)..col.len()], height, false);

            let score = left * right * up * down;

            answer = answer.max(score);
        }
    }

    println!("Part 2: {:?}", answer);
}

fn is_visible(slice: &[u32], height: u32) -> bool {
    for tree in slice {
        if *tree >= height {
            return false;
        }
    }
    true
}

fn view_distance(slice: &[u32], height: u32, reverse: bool) -> usize {
    if slice.len() == 0 {
        return 0;
    }

    let mut d = 0;
    
    if reverse {
        for tree in slice.iter().rev() {
            d += 1;
            if *tree >= height {
                break;
            }
        }
    } else {
        for tree in slice.iter() {
            d += 1;
            if *tree >= height {
                break;
            }
        }
    }

    d
}
