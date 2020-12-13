use utils::{read_strings, Pos};

#[derive(Debug)]
enum Inst {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    L(u32),
    R(u32),
    F(u32),
}

impl From<&str> for Inst {
    fn from(s: &str) -> Self {
        use Inst::*;
        let num = s[1..].parse::<u32>().unwrap();
        match s.chars().nth(0).unwrap() {
            'N' => N(num),
            'S' => S(num),
            'E' => E(num),
            'W' => W(num),
            'L' => L(num),
            'R' => R(num),
            'F' => F(num),
            _ => unreachable!("unexpected instruction"),
        }
    }
}

type Dir = u32;

impl Inst {
    fn mov_ship(&self, pos: &mut Pos, dir: &mut Dir) {
        use Inst::*;
        match self {
            N(v) => {
                pos.y += *v as i64;
            }
            S(v) => {
                pos.y -= *v as i64;
            }
            E(v) => {
                pos.x += *v as i64;
            }
            W(v) => {
                pos.x -= *v as i64;
            }
            L(v) => {
                assert!(0 < *v && *v < 360);
                *dir += 360 - v;
                *dir %= 360;
            }
            R(v) => {
                assert!(0 < *v && *v < 360);
                *dir += v;
                *dir %= 360;
            }
            F(v) => {
                assert_eq!(*dir % 90, 0);
                if *dir == 0 {
                    pos.y += *v as i64;
                } else if *dir == 90 {
                    pos.x += *v as i64;
                } else if *dir == 180 {
                    pos.y -= *v as i64;
                } else if *dir == 270 {
                    pos.x -= *v as i64;
                }
            }
        }
    }

    fn mov_waypoint(&self, ship: &mut Pos, waypoint: &mut Pos) {
        use Inst::*;
        match self {
            N(v) => {
                waypoint.y += *v as i64;
            }
            S(v) => {
                waypoint.y -= *v as i64;
            }
            E(v) => {
                waypoint.x += *v as i64;
            }
            W(v) => {
                waypoint.x -= *v as i64;
            }
            L(v) => {
                assert!(0 < *v && *v < 360);
                assert_eq!(*v % 90, 0);
                for _ in 0..(*v / 90) {
                    let prev = waypoint.clone();
                    waypoint.x = -prev.y;
                    waypoint.y = prev.x;
                }
            }
            R(v) => {
                assert!(0 < *v && *v < 360);
                assert_eq!(*v % 90, 0);
                for _ in 0..(*v / 90) {
                    let prev = waypoint.clone();
                    waypoint.x = prev.y;
                    waypoint.y = -prev.x;
                }
            }
            F(v) => {
                ship.x += waypoint.x * *v as i64;
                ship.y += waypoint.y * *v as i64;
            }
        }
    }
}

fn part1(insts: &[Inst]) {
    let mut pos = Pos { x: 0, y: 0 };
    let mut dir = 90;

    for inst in insts {
        inst.mov_ship(&mut pos, &mut dir);
    }

    println!("Part 1: {}", pos.x.abs() + pos.y.abs());
}

fn part2(insts: &[Inst]) {
    let mut ship = Pos { x: 0, y: 0 };
    let mut waypoint = Pos { x: 10, y: 1 };

    for inst in insts {
        inst.mov_waypoint(&mut ship, &mut waypoint);
    }

    println!("Part 2: {}", ship.x.abs() + ship.y.abs());
}

fn main() {
    let input = read_strings("./day12.txt");
    let insts: Vec<Inst> = input.iter().map(|s| Inst::from(s.as_str())).collect();
    part1(&insts);
    part2(&insts);
}
