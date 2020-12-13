use std::collections::BTreeSet;

use utils::read_strings;

#[derive(Clone)]
enum Inst {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl From<&str> for Inst {
    fn from(s: &str) -> Self {
        use Inst::*;

        let ws: Vec<&str> = s.split(' ').collect();
        match ws[0] {
            "nop" => Nop(ws[1].parse::<i32>().unwrap()),
            "acc" => Acc(ws[1].parse::<i32>().unwrap()),
            "jmp" => Jmp(ws[1].parse::<i32>().unwrap()),
            _ => unreachable!("unknown instruction: {}", ws[0]),
        }
    }
}

#[derive(Clone)]
struct Prog {
    acc: i32,
    pc: u32,
    code: Vec<Inst>,
    visited: BTreeSet<u32>,
}

impl Prog {
    // Perform a computation and returns whether the program terminated or not.
    fn run(&mut self) -> bool {
        while !self.visited.contains(&self.pc) && self.pc < (self.code.len() as u32) {
            self.visited.insert(self.pc);

            match self.code[self.pc as usize] {
                Inst::Nop(_) => {
                    self.pc += 1;
                }
                Inst::Acc(d) => {
                    self.acc += d;
                    self.pc += 1;
                }
                Inst::Jmp(d) => {
                    self.pc = (self.pc as i32 + d) as u32;
                }
            }
        }

        self.pc == (self.code.len() as u32)
    }
}

fn part1() {
    let input = read_strings("./day8.txt");
    let code: Vec<Inst> = input.iter().map(|l| From::<&str>::from(l)).collect();

    let mut prog = Prog {
        acc: 0,
        pc: 0,
        code,
        visited: BTreeSet::new(),
    };
    prog.run();

    println!("Part 1: {}", prog.acc);
}

fn part2() {
    let input = read_strings("./day8.txt");
    let code: Vec<Inst> = input.iter().map(|l| From::<&str>::from(l)).collect();

    let base = Prog {
        acc: 0,
        pc: 0,
        code,
        visited: BTreeSet::new(),
    };

    for i in 0..base.code.len() {
        let mut p = base.clone();

        match base.code[i] {
            Inst::Jmp(x) => {
                p.code[i] = Inst::Nop(x);
            }
            Inst::Nop(x) => {
                p.code[i] = Inst::Jmp(x);
            }
            _ => {
                continue;
            }
        }
        if p.run() {
            println!("Part 2: {}", p.acc);
            return;
        }
    }
    panic!("No answer found");
}

fn main() {
    part1();
    part2();
}
