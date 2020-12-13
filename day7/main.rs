use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};

use utils::read_strings;

#[derive(Clone, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
struct Color {
    adj: String,
    name: String,
}

fn parse(input: &str) -> (Color, Vec<(u32, Color)>) {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref SUBJECT: Regex = Regex::new(r"^(\w+)\s(\w+)\sbag").unwrap();
        static ref OBJECT: Regex = Regex::new(r"(\d+)\s(\w+)\s(\w+)\sbag").unwrap();
    }

    let mut subject: Color = Default::default();
    for cap in SUBJECT.captures_iter(input) {
        subject.adj = cap[1].to_string();
        subject.name = cap[2].to_string();
    }

    let mut edges = vec![];
    for cap in OBJECT.captures_iter(input) {
        let num = cap[1].parse::<u32>().unwrap();
        let adj = cap[2].to_string();
        let name = cap[3].to_string();
        edges.push((num, Color { adj, name }))
    }

    (subject, edges)
}

#[test]
fn test_parse() {
    let ret = parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
    assert_eq!(
        ret,
        (
            Color {
                adj: "light".to_string(),
                name: "red".to_string(),
            },
            vec![
                (
                    1,
                    Color {
                        adj: "bright".to_string(),
                        name: "white".to_string(),
                    }
                ),
                (
                    2,
                    Color {
                        adj: "muted".to_string(),
                        name: "yellow".to_string(),
                    }
                )
            ]
        )
    );
}

fn part1() {
    let input = read_strings("./day7.txt");

    // inner bag -> out bag
    let mut edges: BTreeMap<Color, Vec<Color>> = Default::default();

    for s in input {
        let (outer, inners) = parse(&s);

        for (_, inner) in inners {
            match edges.entry(inner) {
                Entry::Vacant(e) => {
                    e.insert(vec![outer.clone()]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(outer.clone());
                }
            }
        }
    }

    let mut reachable: BTreeSet<Color> = Default::default();
    fn dfs(edges: &BTreeMap<Color, Vec<Color>>, reachable: &mut BTreeSet<Color>, cur: &Color) {
        if let Some(children) = edges.get(&cur) {
            for child in children {
                reachable.insert(child.clone());
                dfs(edges, reachable, child);
            }
        }
    }

    dfs(
        &edges,
        &mut reachable,
        &Color {
            adj: "shiny".to_string(),
            name: "gold".to_string(),
        },
    );

    println!("Part 1: {}", reachable.len());
}

fn part2() {
    let input = read_strings("./day7.txt");

    // outer bag -> inner bags
    let mut edges: BTreeMap<Color, Vec<(u32, Color)>> = Default::default();

    for s in input {
        let (outer, inners) = parse(&s);
        edges.insert(outer, inners);
    }

    fn dfs(edges: &BTreeMap<Color, Vec<(u32, Color)>>, num: u32, cur: &Color) -> u32 {
        if let Some(children) = edges.get(&cur) {
            let mut ans = num;
            for (c_num, child) in children {
                ans += num * dfs(edges, *c_num, child);
            }
            ans
        } else {
            num
        }
    }

    let ans = dfs(
        &edges,
        1,
        &Color {
            adj: "shiny".to_string(),
            name: "gold".to_string(),
        },
    );

    println!("Part 2: {}", ans - 1);
}

fn main() {
    part1();
    part2();
}
