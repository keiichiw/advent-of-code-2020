#![allow(clippy::needless_range_loop)]

use std::collections::BTreeSet;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Pos3 {
    x: i32,
    y: i32,
    z: i32,
}

trait Pos {
    type P;

    fn new(x: i32, y: i32) -> Self::P;
    fn neighbors(&self) -> BTreeSet<Self::P>;
    fn next_active(&self, actives: &BTreeSet<Self::P>) -> bool;
}

impl Pos for Pos3 {
    type P = Pos3;

    fn new(x: i32, y: i32) -> Self::P {
        Pos3 { x, y, z: 0 }
    }

    fn neighbors(&self) -> BTreeSet<Self::P> {
        let mut st = BTreeSet::<Self::P>::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    st.insert(Self::P {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z + dz,
                    });
                }
            }
        }

        st
    }

    fn next_active(&self, actives: &BTreeSet<Self::P>) -> bool {
        let ns = self.neighbors();
        let num = ns.intersection(&actives).count();

        if actives.contains(&self) {
            num == 2 || num == 3
        } else {
            num == 3
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Pos4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Pos for Pos4 {
    type P = Pos4;

    fn new(x: i32, y: i32) -> Self::P {
        Pos4 { x, y, z: 0, w: 0 }
    }

    fn neighbors(&self) -> BTreeSet<Self::P> {
        let mut st = BTreeSet::<Self::P>::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        st.insert(Self::P {
                            x: self.x + dx,
                            y: self.y + dy,
                            z: self.z + dz,
                            w: self.w + dw,
                        });
                    }
                }
            }
        }

        st
    }

    fn next_active(&self, actives: &BTreeSet<Self::P>) -> bool {
        let ns = self.neighbors();
        let num = ns.intersection(&actives).count();

        if actives.contains(&self) {
            num == 2 || num == 3
        } else {
            num == 3
        }
    }
}

trait State {
    type P;

    fn new(grid: &[Vec<char>]) -> Self;
    fn neighbors(&self) -> BTreeSet<Self::P>;
    fn cycle(&mut self);
}

struct StateImpl<T> {
    actives: BTreeSet<T>,
}

impl<T: Pos<P = T> + Clone + Ord> State for StateImpl<T> {
    type P = T;

    fn new(grid: &[Vec<char>]) -> Self {
        let mut actives: BTreeSet<Self::P> = Default::default();
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == '#' {
                    actives.insert(Self::P::new(x as i32, y as i32));
                }
            }
        }

        Self { actives }
    }

    fn neighbors(&self) -> BTreeSet<Self::P> {
        let mut s: BTreeSet<Self::P> = self.actives.clone();
        for pos in self.actives.iter() {
            s = s.union(&pos.neighbors()).cloned().collect();
        }
        s
    }

    fn cycle(&mut self) {
        let mut next_actives = BTreeSet::<Self::P>::new();
        let ps = self.neighbors();

        for p in ps {
            if p.next_active(&self.actives) {
                next_actives.insert(p);
            }
        }

        self.actives = next_actives;
    }
}

fn main() {
    let grid = utils::read_grid("./day17.txt");
    let mut s = StateImpl::<Pos3>::new(&grid);
    for _ in 0..6 {
        s.cycle();
    }
    println!("Part 1: {}", s.actives.len());

    let mut s = StateImpl::<Pos4>::new(&grid);
    for _ in 0..6 {
        s.cycle();
    }
    println!("Part 2: {}", s.actives.len());
}
