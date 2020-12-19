use std::collections::BTreeMap;

use utils::read_strings;

enum Expr {
    Mask([char; 36]),
    Mem { addr: u64, val: u64 },
}

impl From<String> for Expr {
    fn from(s: String) -> Self {
        use lazy_static::lazy_static;
        use regex::Regex;
        use Expr::*;
        lazy_static! {
            static ref MASK: Regex = Regex::new(r"^mask = (\w{36})").unwrap();
            static ref MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }

        if let Some(caps) = MASK.captures(&s) {
            let mut bits = ['X'; 36];
            let cs: Vec<char> = caps.get(1).unwrap().as_str().chars().rev().collect();
            for i in 0..36 {
                bits[i] = cs[i];
            }
            Mask(bits)
        } else if let Some(caps) = MEM.captures(&s) {
            let addr = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Mem { addr, val }
        } else {
            panic!("invalid input: {}", s)
        }
    }
}

struct Val(String);

impl Into<u64> for &Val {
    fn into(self) -> u64 {
        if let Ok(d) = u64::from_str_radix(&self.0, 2) {
            return d;
        }

        //
        let mut ans = 0u64;
        println!("{}", self.0);

        let mut cs: Vec<char> = self.0.chars().collect();
        let idx = cs.iter().position(|&c| c == 'X').unwrap();
        cs[idx] = '0';
        ans += Into::<u64>::into(&Val(cs.iter().collect()));
        cs[idx] = '1';
        ans += Into::<u64>::into(&Val(cs.iter().collect()));
        ans
    }
}

struct State {
    mask: [char; 36],
    mem: BTreeMap<u64, u64>,
}

impl State {
    fn eval(&mut self, expr: &Expr) {
        match *expr {
            Expr::Mask(mask) => {
                self.mask = mask;
            }
            Expr::Mem { addr, val } => {
                let mut v = val;
                for (i, c) in self.mask.iter().enumerate() {
                    if *c == '1' {
                        v |= 1 << i;
                    } else if *c == '0' {
                        v &= !(1 << i);
                    }
                }

                self.mem.insert(addr, v);
            }
        }
    }

    fn eval2(&mut self, expr: &Expr) {
        fn addrs(addr: &Vec<char>) -> Vec<u64> {
            let s: String = addr.iter().collect();
            if let Ok(d) = u64::from_str_radix(&s, 2) {
                return vec![d];
            }

            let mut ret = vec![];
            let mut addr = addr.clone();
            let idx = addr.iter().position(|&c| c == 'X').unwrap();
            addr[idx] = '0';
            ret.append(&mut addrs(&addr));
            addr[idx] = '1';
            ret.append(&mut addrs(&addr));

            ret
        }

        match *expr {
            Expr::Mask(mask) => {
                self.mask = mask;
            }
            Expr::Mem { addr, val } => {
                let mut addr: Vec<char> = format!("{:036b}", addr).chars().collect();
                for (i, c) in self.mask.iter().enumerate() {
                    if *c == '1' {
                        addr[35 - i] = '1';
                    } else if *c == 'X' {
                        addr[35 - i] = 'X';
                    }
                }

                for dst in addrs(&addr) {
                    self.mem.insert(dst, val);
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        let mut ans = 0u64;
        for (_, v) in self.mem.iter() {
            ans += v;
        }
        ans
    }
}

fn main() {
    let lines = read_strings("./day14.txt");
    let exprs: Vec<Expr> = lines
        .iter()
        .map(|l| From::<String>::from(l.to_string()))
        .collect();

    let mut s = State {
        mask: ['X'; 36],
        mem: Default::default(),
    };
    for e in &exprs {
        s.eval(e);
    }

    println!("Part 1: {}", s.sum());

    let mut s = State {
        mask: ['X'; 36],
        mem: Default::default(),
    };
    for e in &exprs {
        s.eval2(e);
    }

    println!("Part 2: {}", s.sum());
}
