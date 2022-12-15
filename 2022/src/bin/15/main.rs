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

#[derive(Debug)]
struct Sensor {
    pub coords: (i32, i32),
    pub radius: i32,
}

impl Sensor {
    pub fn get_range_in_radius(&self, row: i32) -> Option<(i32, i32)> {
        let (x, y) = self.coords;
        let d_y = y.abs_diff(row) as i32;
        let d_x = self.radius - d_y;

        if d_x >= 0 {
            Some(((x - d_x), (x + d_x)))
        } else {
            None
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let sensors = input
        .lines()
        .map(extract_coords)
        .map(|((x_s, y_s), (x_b, y_b))| {
            let distance = x_s.abs_diff(x_b) + y_s.abs_diff(y_b);
            Sensor {
                coords: (x_s, y_s),
                radius: distance as i32,
            }
        })
        .collect::<Vec<_>>();

    let row = 2_000_000;
    let ranges = sensors
        .iter()
        .filter_map(|s| s.get_range_in_radius(row))
        .collect::<Vec<_>>();

    let ranges = merge_ranges(ranges);

    let answer = ranges
        .iter()
        .map(|(from, to)| from.abs_diff(*to))
        .sum::<u32>();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let sensors = input
        .lines()
        .map(extract_coords)
        .map(|((x_s, y_s), (x_b, y_b))| {
            let distance = x_s.abs_diff(x_b) + y_s.abs_diff(y_b);
            Sensor {
                coords: (x_s, y_s),
                radius: distance as i32,
            }
        })
        .collect::<Vec<_>>();

    let l = 0;
    let u = 4_000_000;
    let (mut x, mut y) = (0, 0);

    // I have no excuse for this
    for row in l..=u {
        let ranges = sensors
            .iter()
            .filter_map(|s| s.get_range_in_radius(row))
            .map(|(from, to)| (from.clamp(l, u), to.clamp(l, u)))
            .collect::<Vec<_>>();

        let ranges = merge_ranges(ranges);

        if ranges.len() > 1 {
            x = ranges[0].1 + 1;
            y = row;
        }
    }

    let answer = x as i64 * u as i64 + y as i64;
    println!("Part 2: {:?}", answer);
}

fn extract_coords(line: &str) -> ((i32, i32), (i32, i32)) {
    let coords = line
        .split(": ")
        .map(|s| {
            s.split(", ")
                .map(|s| s.split("=").last().unwrap().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sensor = (coords[0][0], coords[0][1]);
    let beacon = (coords[1][0], coords[1][1]);

    (sensor, beacon)
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_unstable();

    let mut merged = vec![];
    merged.push(ranges[0]);

    for idx in 1..ranges.len() {
        let current = ranges[idx];
        let last = merged.last_mut().unwrap();

        if (last.0..=last.1).contains(&current.0) {
            last.1 = last.1.max(current.1);
        } else {
            merged.push(current)
        }
    }

    merged
}
