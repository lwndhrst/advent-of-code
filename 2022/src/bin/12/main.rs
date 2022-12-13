#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
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
    let path = PathBuf::from("src/bin/12/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let grid = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    let mut goal = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                'S' => start = (i, j),
                'E' => goal = (i, j),
                _ => {}
            }
        }
    }

    let path = bfs(&grid, start, goal);

    let answer = path.len();
    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let grid = input
        .split(new_line::SINGLE)
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    let mut goal = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                'S' => start = (i, j),
                'E' => goal = (i, j),
                _ => {}
            }
        }
    }

    let mut shortest = bfs(&grid, start, goal).len();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'a' {
                let path = bfs(&grid, (i, j), goal).len();
                if path > 0 {
                    shortest = shortest.min(path);
                }
            }
        }
    }

    let answer = shortest;
    println!("Part 2: {:?}", answer);
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> Vec<(usize, usize)> {
    let mut explored = HashMap::new();
    let mut queue = VecDeque::new();

    explored.insert(start, start);
    queue.push_front(start);

    let mut path = vec![];

    while !queue.is_empty() {
        let current = queue.pop_back().unwrap();
        if current == goal {
            let mut pos = goal;
            while pos != start {
                path.push(pos);
                pos = explored[&pos];
            }
        }

        let (i, j) = current;
        let dirs = if i <= 0 || i >= grid.len() - 1 || j >= grid[i].len() - 1 {
            continue;
        } else if j <= 0 {
            vec![(i, j + 1)]
        } else {
            let u = (i - 1, j);
            let r = (i, j + 1);
            let d = (i + 1, j);
            let l = (i, j - 1);
            vec![u, r, d, l]
        };

        for dir in dirs {
            if is_allowed(grid[i][j], grid[dir.0][dir.1]) && !explored.contains_key(&dir) {
                explored.insert(dir, current);
                queue.push_front(dir);
            }
        }
    }

    path
}

fn is_allowed(from: char, to: char) -> bool {
    let from = if from == 'S' {
        'a' as u8
    } else {
        from as u8
    };

    let to = if to == 'E' {
        'z' as u8
    } else {
        to as u8
    };

    to < from + 2
}
