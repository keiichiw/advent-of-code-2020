use std::collections::BTreeMap;
use std::fs::read_to_string;

use regex::Regex;
#[macro_use]
extern crate lazy_static;

fn validate_kv(k: &str, v: &str) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    match k {
        "byr" | "iyr" | "eyr" => {
            if v.len() != 4 {
                return false;
            }

            if let Ok(d) = v.parse::<u32>() {
                let (mn, mx) = if k == "byr" {
                    (1920, 2002)
                } else if k == "iyr" {
                    (2010, 2020)
                } else {
                    (2020, 2030)
                };
                mn <= d && d <= mx
            } else {
                false
            }
        }
        "hgt" => {
            if v.ends_with("cm") {
                if let Ok(d) = v[0..(v.len() - 2)].parse() {
                    150 <= d && d <= 193
                } else {
                    false
                }
            } else if v.ends_with("in") {
                if let Ok(d) = v[0..(v.len() - 2)].parse() {
                    59 <= d && d <= 76
                } else {
                    false
                }
            } else {
                false
            }
        }
        "hcl" => HCL_RE.is_match(v),
        "ecl" => ECL_RE.is_match(v),
        "pid" => PID_RE.is_match(v),
        _ => unreachable!(format!("key:{}, val:{}", k, v)),
    }
}

fn read_dicts(path: &str) -> Vec<BTreeMap<String, String>> {
    let mut vec = vec![];
    let s = read_to_string(path).unwrap();
    for s in s.split("\n\n") {
        let mut mp = BTreeMap::new();
        for elem in s.split(|d| d == ' ' || d == '\n') {
            if elem.is_empty() {
                continue;
            }
            let mut it = elem.split(':');
            let key = it.next().unwrap();
            let val = it.next().unwrap();
            mp.insert(key.to_string(), val.to_string());
        }
        vec.push(mp);
    }
    vec
}

const KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn validate_keys(dict: &BTreeMap<String, String>) -> bool {
    KEYS.iter().all(|k| dict.get(&k.to_string()).is_some())
}

fn validate_values(dict: &BTreeMap<String, String>) -> bool {
    KEYS.iter().all(|k| {
        if let Some(v) = dict.get(&k.to_string()) {
            validate_kv(k, v)
        } else {
            false
        }
    })
}

fn solve() {
    let mut part1 = 0;
    let mut part2 = 0;

    let dicts = read_dicts("./day4.txt");

    for d in dicts {
        if validate_keys(&d) {
            part1 += 1;

            if validate_values(&d) {
                part2 += 1;
            }
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn main() {
    solve();
}
