use utils::read_strings;

fn part1() {
    let input = read_strings("./day13.txt");
    let depart = input[0].parse::<u64>().unwrap();
    let buses: Vec<u64> = input[1]
        .split(',')
        .filter(|c| c != &"x")
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    let mut wait_time = depart;
    let mut ans = 0;
    for bus in buses {
        assert_ne!(depart % bus, 0);

        let time = ((depart / bus) + 1) * bus;
        let diff = time - depart;

        if diff < wait_time {
            ans = bus * diff;
            wait_time = diff;
        }
    }

    println!("Part 1: {}", ans);
}

#[allow(clippy::many_single_char_names)]
fn ext_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b != 0 {
        d = ext_gcd(b, a % b, y, x);
        *y -= (a / b) * *x;
    } else {
        *x = 1;
        *y = 0;
    }
    d
}

#[test]
fn test_ext_gcd() {
    let (a, b) = (1071, 1029);
    let (mut x, mut y) = (0, 0);
    let d = ext_gcd(a, b, &mut x, &mut y);
    assert_eq!(a * x + b * y, d);
}

fn solve(arr: &[(i64, i64)]) -> i64 {
    let mut multi = 1;
    for (x, _) in arr {
        multi *= x;
    }

    let mut acc = 0i64;
    for (a_i, i) in arr {
        let b_i = multi / a_i;

        // Find (x_i, y_i) where a_i * x_i + b_i * y_i = 1 holds.
        let (mut x_i, mut y_i) = (0, 0);
        let d_i = ext_gcd(*a_i, b_i, &mut x_i, &mut y_i);
        assert_eq!(d_i, 1);
        assert_eq!(a_i * x_i + b_i * y_i, 1);

        //     b_i * y_i ≡ 1 (mod a_i)
        // => b_i * y_i * i ≡ i (mod a_i)
        // Then sum {b_j * y_j * j | for each j } ≡ i (mod a_i) because if i != j, b_j % a_i == 0 holds.
        acc += b_i * y_i * *i;
    }

    if acc > 0 {
        acc %= multi;
    } else {
        unimplemented!();
    }

    acc
}

#[test]
fn test_solve() {
    assert_eq!(solve(&[(17, 0), (13, 13 - 2), (19, 19 - 3)]), 3417);
}

fn part2() {
    let input = read_strings("./day13.txt");
    let mut buses = vec![];
    for (index, s) in input[1].split(',').enumerate() {
        if s == "x" {
            continue;
        }

        let bus = s.parse::<i64>().unwrap();
        buses.push((bus, bus - index as i64));
    }

    let ans = solve(&buses);
    println!("Part 2: {}", ans);
}

fn main() {
    part1();
    part2();
}
