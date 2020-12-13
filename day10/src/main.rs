use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use utils::read_nums;

// `nums` must be sorted.
fn part1(nums: &Vec<u64>) {
    let mut cur = 0;
    let mut diffs: BTreeMap<u64, u64> = Default::default();

    for x in nums {
        let d = x - cur;
        cur = *x;
        match diffs.entry(d) {
            Entry::Vacant(e) => {
                e.insert(1);
            }
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
        }
    }

    let one = diffs.get(&1).unwrap();
    let three = diffs.get(&3).unwrap();

    println!("Part 1: {}", one * three);
}

fn part2(nums: &Vec<u64>) {
    let mut dp: Vec<u128> = Default::default();
    dp.resize(nums[nums.len() - 1] as usize + 1, 0);

    dp[0] = 1;

    for x in nums {
        for i in (std::cmp::max(0, (*x as i64) - 3) as usize)..(*x as usize) {
            dp[*x as usize] += dp[i];
        }
    }

    println!("Part 2: {}", dp.last().unwrap());
}

fn main() {
    let mut nums = read_nums::<u64>("./day10.txt");
    nums.sort();
    let goal = nums[nums.len() - 1] + 3;
    nums.push(goal);

    part1(&nums);
    part2(&nums);
}
