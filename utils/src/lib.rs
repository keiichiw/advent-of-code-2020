use std::fs;
use std::fs::File;
use std::str::FromStr;

pub fn read_strings(path: &str) -> Vec<String> {
    let inp = fs::read_to_string(path).expect("failed to read input file");
    inp.trim().split('\n').map(|x| x.to_string()).collect()
}

pub fn read_nums<T: FromStr>(path: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let inp = fs::read_to_string(path).expect("failed to read input file");
    inp.trim()
        .split('\n')
        .map(|s| s.parse::<T>().expect("failed to parse"))
        .collect()
}

pub fn read_grid(path: &str) -> Vec<Vec<char>> {
    read_strings(path)
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

use std::io::{BufRead, BufReader};

/// Reads a list of strings separated by an empty line.
pub fn read_string_lists(path: &str) -> Vec<Vec<String>> {
    let file = File::open(path).expect("failed to open input file");

    let mut ret = vec![];
    let mut cur = vec![];
    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        if l.is_empty() {
            ret.push(cur);
            cur = vec![];
        } else {
            cur.push(l);
        }
    }
    ret.push(cur);

    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
