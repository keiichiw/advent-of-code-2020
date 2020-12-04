use utils::read_grid;

fn solve(grid: &Vec<Vec<char>>, right: usize, down: usize) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut x = 0;
    let mut ans = 0;
    for i in (0..height).step_by(down) {
        if grid[i][x] == '#' {
            ans += 1;
        }
        x = (x + right) % width;
    }

    ans
}

fn part1() {
    let grid: Vec<Vec<char>> = read_grid("./input.txt");
    let ans = solve(&grid, 3, 1);
    println!("Part 1: {:?}", ans);
}

fn part2() {
    let grid: Vec<Vec<char>> = read_grid("./input.txt");
    let params = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut ans = 1;

    for (x, y) in params.iter() {
        ans *= solve(&grid, *x, *y);
    }

    println!("Part 2: {:?}", ans);
}

fn main() {
    part1();
    part2();
}
