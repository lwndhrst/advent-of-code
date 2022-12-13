#![allow(dead_code)]

use std::cmp::Ordering;
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
    let path = PathBuf::from("src/bin/13/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let answer = input
        .split(new_line::DOUBLE)
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, input)| (i, split_package(input)))
        .filter(|(_, (l, r))| check_order(l, r))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let mut input = input.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    let dividers = vec!["[[2]]", "[[6]]"];
    input.append(&mut dividers.clone());

    input.sort_by(|a, b| compare(*a, *b));

    let answer = input
        .iter()
        .enumerate()
        .filter(|(_, s)| dividers.contains(*s))
        .map(|(i, _)| i + 1)
        .product::<usize>();

    println!("Part 2: {:?}", answer);
}

fn split_package(input: &str) -> (&str, &str) {
    let mut lines = input.lines();
    let l = lines.next().unwrap();
    let r = lines.next().unwrap();
    (l, r)
}

fn check_order(l: &str, r: &str) -> bool {
    match compare(l, r) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => unreachable!(),
    }
}

fn compare(l: &str, r: &str) -> Ordering {
    let mut l = parse_tokens(l);
    let mut r = parse_tokens(r);

    while let (Some(l_token), Some(r_token)) = (l.next(), r.next()) {
        let order = match (l_token, r_token) {
            (Token::Num(a), Token::Num(b)) => a.cmp(&b),
            (Token::Num(a), Token::Open) => compare_num_to_list(a, &mut r),
            (Token::Open, Token::Num(b)) => compare_num_to_list(b, &mut l).reverse(),
            (Token::Open, Token::Open) => Ordering::Equal,
            (Token::Close, Token::Close) => Ordering::Equal,
            (Token::Close, _) => Ordering::Less,
            (_, Token::Close) => Ordering::Greater,
        };

        match order {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }

    if l.count() > r.count() {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn compare_num_to_list(num: u32, list: &mut impl Iterator<Item = Token>) -> Ordering {
    let mut depth = 1;
    while depth > 0 {
        match list.next().unwrap() {
            Token::Open => depth += 1,
            Token::Close => return Ordering::Greater,
            Token::Num(x) if num < x => return Ordering::Less,
            Token::Num(x) if num > x => return Ordering::Greater,
            Token::Num(_) => break,
        }
    }

    while depth > 0 {
        match list.next().unwrap() {
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            Token::Num(_) => return Ordering::Less,
        }
    }

    Ordering::Equal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Num(u32),
}

fn parse_tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut acc = None;
    input
        .as_bytes()
        .windows(2)
        .map(move |w| scan_token(&mut acc, w[0], w[1]))
        .filter_map(|x| x)
        .chain(std::iter::once(Token::Close))
}

fn scan_token(acc: &mut Option<u32>, current_symbol: u8, next_symbol: u8) -> Option<Token> {
    match current_symbol {
        b'[' => Some(Token::Open),
        b']' => Some(Token::Close),
        b',' => None,
        x if x.is_ascii_digit() => {
            let digit = x as u32;
            let value = acc.take().unwrap_or(0) * 10 + digit;
            if next_symbol.is_ascii_digit() {
                *acc = Some(value);
                None
            } else {
                Some(Token::Num(value))
            }
        }
        _ => unreachable!(),
    }
}
