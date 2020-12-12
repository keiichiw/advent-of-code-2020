use std::collections::BTreeSet;

use utils::read_string_lists;

fn part1() {
    let slists = read_string_lists("./day6.txt");

    let mut ans = 0;

    for group in slists {
        let mut st: BTreeSet<char> = Default::default();
        for l in group {
            for c in l.chars() {
                st.insert(c);
            }
        }

        ans += st.len();
    }

    println!("Part 1: {}", ans);
}

fn part2() {
    let slists = read_string_lists("./day6.txt");

    let mut ans = 0;

    for group in slists {
        if group.len() == 1 {
            ans += group[0].len();
            continue;
        }

        let mut base: BTreeSet<char> = Default::default();
        for c in group[0].chars() {
            base.insert(c);
        }

        for l in &group[1..] {
            let mut st: BTreeSet<char> = Default::default();
            for c in l.chars() {
                st.insert(c);
            }

            base = base.intersection(&st).cloned().collect();
        }

        ans += base.len();
    }

    println!("Part 2: {}", ans);
}

fn main() {
    part1();
    part2();
}
