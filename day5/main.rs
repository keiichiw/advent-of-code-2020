use utils::read_grid;

fn seat(line: &Vec<char>) -> (u32, u32) {
    assert_eq!(line.len(), 10);

    // Row
    let (mut f, mut b) = (0, 128);
    let mut med = (f + b) / 2;

    for i in 0..6usize {
        if line[i] == 'F' {
            b = med;
        } else if line[i] == 'B' {
            f = med;
        } else {
            unreachable!(line[i]);
        }
        med = (f + b) / 2;
    }
    let row = if line[6] == 'F' { f } else { b - 1 };

    // Column
    let (mut l, mut r) = (0, 8);
    let mut med = (l + r) / 2;

    for i in 7..9usize {
        if line[i] == 'L' {
            r = med;
        } else if line[i] == 'R' {
            l = med;
        } else {
            unreachable!(line[i]);
        }
        med = (l + r) / 2;
    }
    let col = if line[9] == 'L' { l } else { r - 1 };

    (row, col)
}

#[test]
fn test_seat() {
    assert_eq!(seat(&"FBFBBFFRLR".chars().collect::<Vec<char>>()), (44, 5));
}

fn main() {
    let grid = read_grid("./day5.txt");
    let mut ans = 0;

    let mut ids = vec![];

    for line in grid {
        let (r, c) = seat(&line);
        let t = r * 8 + c;
        ids.push(t);
        if ans < t {
            ans = t;
        }
    }
    println!("part1: {}", ans);

    ids.sort();

    for i in 0..(ids.len() - 1) {
        if ids[i] + 1 < ids[i + 1] {
            println!("part2: {}", ids[i] + 1);
        }
    }
}
