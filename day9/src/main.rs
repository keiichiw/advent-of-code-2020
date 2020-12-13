use std::collections::{BTreeSet, VecDeque};

use utils::read_nums;

fn part1(nums: &Vec<u64>) -> u64 {
    let mut st: BTreeSet<u64> = Default::default();
    let mut deq: VecDeque<u64> = Default::default();
    for i in 0..25 {
        deq.push_back(nums[i]);
        st.insert(nums[i]);
    }

    for i in 25..nums.len() {
        let n = nums[i];

        let mut flg = false;
        for x in deq.iter() {
            if *x >= n {
                continue;
            }
            let y = n - x;
            if y != *x && st.contains(&y) {
                flg = true;
                break;
            }
        }

        if !flg {
            println!("Part 1: {}", n);
            return n;
        }

        let hd = deq.pop_front().unwrap();
        st.remove(&hd);
        deq.push_back(n);
        st.insert(n);
    }
    panic!("No answer");
}

fn part2(nums: &Vec<u64>, target: u64) {
    let mut deq: VecDeque<u64> = Default::default();
    let mut sum = 0;
    for n in nums {
        sum += n;
        deq.push_back(*n);

        while sum > target {
            let hd = deq.pop_front().unwrap();
            sum -= hd;
        }

        if sum == target {
            let min = deq.iter().min().unwrap();
            let max = deq.iter().max().unwrap();
            println!("Part 2: {}", min + max);
            return;
        }
    }
    panic!("No answer");
}

fn main() {
    let nums = read_nums::<u64>("./day9.txt");
    let x = part1(&nums);
    part2(&nums, x);
}
