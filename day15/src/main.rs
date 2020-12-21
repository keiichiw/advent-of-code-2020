use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

struct State {
    turn: u32,
    mp: BTreeMap<u32, u32>,
}

impl State {
    fn new(init: &[u32]) -> (Self, u32) {
        let mut s = State {
            turn: 1,
            mp: Default::default(),
        };
        let mut r = 0;
        for x in init {
            r = s.add(*x);
        }
        (s, r)
    }

    fn add(&mut self, x: u32) -> u32 {
        self.turn += 1;
        match self.mp.entry(x) {
            Entry::Vacant(e) => {
                e.insert(self.turn);
                0
            }
            Entry::Occupied(mut e) => {
                let last = e.get();
                let res = self.turn - last;
                *e.get_mut() = self.turn;

                res
            }
        }
    }
}

fn main() {
    let inp = vec![1, 17, 0, 10, 18, 11, 6];
    //    let inp = vec![0, 3, 6];
    let (mut s, mut x) = State::new(&inp);

    while s.turn < 2020 {
        x = s.add(x);
    }
    println!("Turn {}: {}", s.turn, x);

    while s.turn < 30000000 {
        x = s.add(x);
    }
    println!("Turn {}: {}", s.turn, x);
}
