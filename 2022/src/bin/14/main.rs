#![allow(dead_code)]

use std::cmp::Ordering;

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
    part_two(&input);
}

fn part_one(input: &str) {
    let mut grid = build_grid(input);
    let src = (500, 0);

    let finish_condition = |y, grid: &Vec<Vec<char>>| y + 1 >= grid.len();
    let answer = simulate(&mut grid, src, finish_condition);

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let mut grid = build_grid(input);
    for row in &mut grid {
        row.append(&mut vec!['.'; 1000 - row.len()])
    }

    grid.push(vec!['.'; grid[0].len()]);
    grid.push(vec!['#'; grid[0].len()]);

    let src = (500, 0);

    let finish_condition = |_, grid: &Vec<Vec<char>>| grid[0][500] == 'o';
    let answer = simulate(&mut grid, src, finish_condition);

    println!("Part 2: {:?}", answer);
}

fn simulate(
    grid: &mut Vec<Vec<char>>,
    src: (usize, usize),
    finish_condition: fn(usize, &Vec<Vec<char>>) -> bool,
) -> usize {
    let mut count = 0;

    loop {
        let (mut x, mut y) = src;

        let mut settled = false;
        while !settled {
            if finish_condition(y, grid) {
                return count;
            }

            if grid[y + 1][x] == '.' {
                y += 1;
                continue;
            } else {
                if grid[y + 1][x - 1] == '.' {
                    x -= 1;
                    y += 1;
                    continue;
                } else if grid[y + 1][x + 1] == '.' {
                    x += 1;
                    y += 1;
                    continue;
                } else {
                    grid[y][x] = 'o';
                    settled = true;
                    count += 1;
                }
            }
        }
    }
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    let paths = input
        .lines()
        .map(|s| s.split(" -> ").map(into_coords).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut bounds = (0, 0);
    for path in &paths {
        for coords in path {
            bounds.0 = (bounds.0).max(coords.0);
            bounds.1 = (bounds.1).max(coords.1);
        }
    }

    let mut grid = Vec::with_capacity(bounds.1 + 1);
    for _ in 0..=bounds.1 {
        grid.push(vec!['.'; bounds.0 + 1]);
    }

    for path in paths {
        for window in path.windows(2) {
            let (x_0, y_0) = window[0];
            let (x_1, y_1) = window[1];

            if y_0 == y_1 {
                match x_0.cmp(&x_1) {
                    Ordering::Less => {
                        let slice = &mut grid[y_0][x_0..=x_1];
                        slice.fill('#');
                    }
                    Ordering::Greater => {
                        let slice = &mut grid[y_0][x_1..=x_0];
                        slice.fill('#');
                    }
                    Ordering::Equal => unreachable!(),
                }
            } else {
                match y_0.cmp(&y_1) {
                    Ordering::Less => {
                        for idx in y_0..=y_1 {
                            grid[idx][x_0] = '#';
                        }
                    }
                    Ordering::Greater => {
                        for idx in y_1..=y_0 {
                            grid[idx][x_0] = '#';
                        }
                    }
                    Ordering::Equal => unreachable!(),
                }
            }
        }
    }

    grid
}

fn into_coords(s: &str) -> (usize, usize) {
    let mut coords = s.split(",").map(|s| s.parse::<usize>().unwrap());
    let x = coords.next().unwrap();
    let y = coords.next().unwrap();
    (x, y)
}
