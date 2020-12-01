use std::collections::BTreeSet;
use std::iter::FromIterator;

use utils::read_nums;

fn find_pair(arr: &[u32], target: u32) -> Option<(u32, u32)> {
    let st = BTreeSet::from_iter(arr.iter());

    for x in arr {
        let y = target - x;
        if st.contains(&y) {
            return Some((*x, y));
        }
    }

    None
}

fn find_triple(arr: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    let st = BTreeSet::from_iter(arr.iter());

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            let x = arr[i];
            let y = arr[j];
            if x + y > target {
                continue;
            }
            let z = target - (x + y);

            if st.contains(&z) {
                return Some((x, y, z));
            }
        }
    }

    None
}

fn main() {
    let inp: Vec<u32> = read_nums("./input.txt");

    let (x, y) = find_pair(&inp, 2020).unwrap();
    println!("Day 1, Part 1: {} ({}, {})", x * y, x, y);

    let (x, y, z) = find_triple(&inp, 2020).unwrap();
    println!("Day 1, Part 2: {} ({}, {}, {})", x * y * z, x, y, z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let l = [1721, 979, 366, 299, 675, 1456];
        let (x, y) = find_pair(&l, 2020).unwrap();
        assert_eq!(x * y, 514579);
    }
}
