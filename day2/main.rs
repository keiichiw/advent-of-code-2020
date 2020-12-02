use utils::read_strings;

fn part1() {
    let inp: Vec<String> = read_strings("./input.txt");
    let mut ans = 0;
    for s in inp {
        let v: Vec<&str> = s.split(' ').collect();
        let (min, max) = {
            let nums: Vec<&str> = v[0].split('-').collect();
            (
                nums[0].parse::<u32>().unwrap(),
                nums[1].parse::<u32>().unwrap(),
            )
        };
        let ch = v[1].chars().nth(0).unwrap();
        let pass = v[2];

        let mut cnt = 0;
        for c in pass.chars() {
            if c == ch {
                cnt += 1;
            }
        }

        if min <= cnt && cnt <= max {
            ans += 1;
        }
    }

    println!("Part 1: {}", ans);
}

fn part2() {
    let inp: Vec<String> = read_strings("./input.txt");
    let mut ans = 0;
    for s in inp {
        let v: Vec<&str> = s.split(' ').collect();
        let (min, max) = {
            let nums: Vec<&str> = v[0].split('-').collect();
            (
                nums[0].parse::<usize>().unwrap(),
                nums[1].parse::<usize>().unwrap(),
            )
        };
        let ch = v[1].chars().nth(0).unwrap();
        let pass = v[2];

        if (pass.chars().nth(min - 1).unwrap() == ch) ^ (pass.chars().nth(max - 1).unwrap() == ch) {
            ans += 1;
        }
    }

    println!("Part 2: {}", ans);
}

fn main() {
    part1();
    part2();
}
