use utils::read_grid;

fn solve<F>(board: &[Vec<char>], count_neighbor: F, threshold: u32) -> u32
where
    F: Fn(&[Vec<char>], usize, usize) -> u32,
{
    let mut cur = board.to_owned();
    let mut updated = true;
    while updated {
        updated = false;
        let mut next = cur.clone();

        for i in 0..cur.len() {
            for j in 0..cur[0].len() {
                if cur[i][j] == '.' {
                    continue;
                }

                let cnt = count_neighbor(&cur, i, j);
                if cur[i][j] == 'L' && cnt == 0 {
                    updated = true;
                    next[i][j] = '#';
                } else if cur[i][j] == '#' && cnt >= threshold {
                    updated = true;
                    next[i][j] = 'L';
                }
            }
        }
        cur = next;
    }

    let mut ans = 0;
    for l in cur {
        for c in l {
            if c == '#' {
                ans += 1;
            }
        }
    }

    ans
}

fn part1(board: &[Vec<char>]) {
    fn count_neighbor(board: &[Vec<char>], x: usize, y: usize) -> u32 {
        let valid = |i: i32, j: i32| {
            0 <= i && i < (board.len() as i32) && 0 <= j && j < (board[0].len() as i32)
        };

        let mut cnt = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx, dy) == (0, 0) {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if valid(nx, ny) && board[nx as usize][ny as usize] == '#' {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    let ans = solve(board, count_neighbor, 4);
    println!("Part 1: {}", ans);
}

fn part2(board: &[Vec<char>]) {
    fn count_neighbor(board: &[Vec<char>], x: usize, y: usize) -> u32 {
        let valid = |i: i32, j: i32| {
            0 <= i && i < (board.len() as i32) && 0 <= j && j < (board[0].len() as i32)
        };

        let mut cnt = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx, dy) == (0, 0) {
                    continue;
                }
                let mut nx = x as i32 + dx;
                let mut ny = y as i32 + dy;
                while valid(nx, ny) {
                    let c = board[nx as usize][ny as usize];
                    if c != '.' {
                        if c == '#' {
                            cnt += 1;
                        }
                        break;
                    }
                    nx += dx;
                    ny += dy;
                }
            }
        }
        cnt
    }
    let ans = solve(board, count_neighbor, 5);
    println!("Part 2: {}", ans);
}

fn main() {
    let board = read_grid("./day11.txt");
    part1(&board);
    part2(&board);
}
