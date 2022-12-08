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
enum Node {
    Dir(Dir),
    File(File),
}

impl Node {
    pub fn name(&self) -> String {
        match self {
            Node::Dir(dir) => dir.name.to_owned(),
            Node::File(file) => file.name.to_owned(),
        }
    }

    pub fn from_line(input: &str) -> Self {
        if input.starts_with("dir") {
            Self::Dir(Dir::from_lines(input))
        } else {
            Self::File(File::from_lines(input))
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Node::Dir(dir) => dir.size(),
            Node::File(file) => file.size,
        }
    }
} 

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<Node>,
}

impl Dir {
    pub fn from_commands(commands: Vec<Command>) -> Self {
        let mut root = Dir { name: "/".to_owned(), children: vec![] };
        let mut path = vec![];

        for command in commands {
            if command.cmd.starts_with("cd") {
                let cmd = command.cmd;
                if cmd.ends_with("..") {
                    path.pop();
                } else if cmd.ends_with("/") {
                    path.clear();
                } else {
                    path.push(cmd[3..].to_owned());
                }
            } else {
                for node in command.output.unwrap() {
                    let mut path = path.clone();
                    path.reverse();
                    root.add_child(Node::from_line(&node), path);
                }
            }
        }

        root
    }

    pub fn from_lines(input: &str) -> Self {
        let name = input.split(" ").last().unwrap();

        Self {
            name: name.to_owned(),
            children: vec![],
        }
    }

    pub fn add_child(&mut self, node: Node, mut path: Vec<String>) {
        if path.len() == 0 {
            if self.contains(&node) {
                return;
            }

            self.children.push(node);
            return;
        }

        let name = path.pop().unwrap();
        for child in self.children.iter_mut() {
            if let Node::Dir(ref mut dir) = child {
                if dir.name == name {
                    dir.add_child(node, path);
                    break;
                }
            }
        }
    }

    pub fn contains(&self, node: &Node) -> bool {
        for child in &self.children {
            if child.name() == node.name() {
                return true;
            }
        }

        false
    }

    pub fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|n| n.size())
            .sum()
    }

    pub fn part_one(&self) -> u32 {
        let mut sum = 0;

        for child in &self.children {
            match child {
                Node::Dir(dir) => {
                    let size = dir.size();
                    if size <= 100_000 {
                        sum += size;
                    }
                    sum += dir.part_one();
                },
                _ => {},
            }
        }

        sum
    }

    pub fn part_two(&self, min: u32, unused: u32) -> u32 {
        let mut min = min;

        for child in &self.children {
            match child {
                Node::Dir(dir) => {
                    let size = dir.size();
                    if size >= 30_000_000 - unused {
                        min = min.min(size);
                    }
                    min = min.min(dir.part_two(min, unused));
                },
                _ => {},
            }
            
        }

        min
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn from_lines(input: &str) -> Self {
        let mut iter = input.split(" ");
        let size = iter.next().unwrap().parse::<u32>().unwrap();
        let name = iter.next().unwrap();

        Self {
            name: name.to_owned(),
            size,
        }
    }
}

#[derive(Debug)]
struct Command {
    cmd: String,
    output: Option<Vec<String>>,
}

impl Command {
    pub fn from_lines(input: &str) -> Self {
        let lines = input
            .split(new_line::SINGLE)
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();
        
        let cmd = lines[0].to_owned();
        let output = if lines.len() > 1 {
            Some((&lines[1..]).to_owned().iter().map(|s| s.to_string()).collect())
        } else {
            None
        };

        Self { cmd, output }
    }
}

fn main() {
    let path = PathBuf::from("src/bin/07/input.txt");
    let input = read_to_string(path).unwrap();

    run(&input);
}

fn run(input: &str) {
    let commands = input
        .split("$ ")
        .filter(|s| s.len() > 0)
        .map(|s| Command::from_lines(s))
        .collect::<Vec<Command>>();

    let fs = Dir::from_commands(commands);

    let answer = fs.part_one();
    println!("Part 1: {:?}", answer);

    let answer = fs.part_two(u32::MAX, 70_000_000 - fs.size());
    println!("Part 2: {:?}", answer);
}
