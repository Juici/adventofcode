#![feature(array_windows)]
#![feature(iter_map_windows)]

const INPUT: &str = include_str!("./input");

fn main() {
    part1();
    part2();
}

fn part1() {
    fn is_xmas_window(window: &[char; 4]) -> bool {
        window == &['X', 'M', 'A', 'S'] || window == &['S', 'A', 'M', 'X']
    }

    let lines = INPUT.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut count = 0;

    // Rows.
    count += lines
        .iter()
        .map(|line| line.array_windows::<4>().filter(|w| is_xmas_window(w)).count())
        .sum::<usize>();

    // Columns.
    count += (0..width)
        .map(|col| lines.iter().map(|l| l[col]).map_windows(is_xmas_window).filter(|b| *b).count())
        .sum::<usize>();

    // Right diagonals.
    count += (0..height)
        .map(|row| {
            (row..height)
                .zip(0..width)
                .map(|(row, col)| lines[row][col])
                .map_windows(is_xmas_window)
                .filter(|b| *b)
                .count()
        })
        .chain((1..width).map(|col| {
            (0..height)
                .zip(col..width)
                .map(|(row, col)| lines[row][col])
                .map_windows(is_xmas_window)
                .filter(|b| *b)
                .count()
        }))
        .sum::<usize>();

    // Left diagonals.
    count += (0..width)
        .map(|col| {
            (0..height)
                .zip((0..=col).rev())
                .map(|(row, col)| lines[row][col])
                .map_windows(is_xmas_window)
                .filter(|b| *b)
                .count()
        })
        .chain((1..height).map(|row| {
            (row..height)
                .zip((0..width).rev())
                .map(|(row, col)| lines[row][col])
                .map_windows(is_xmas_window)
                .filter(|b| *b)
                .count()
        }))
        .sum::<usize>();

    println!("part1: {count}");
}

fn part2() {
    let lines = INPUT.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    if height < 3 || width < 3 {
        println!("part2: 0");
        return;
    }

    let mut count = 0;

    for window in lines.array_windows::<3>() {
        for col in 0..(width - 2) {
            let mid = window[1][col + 1];

            if mid != 'A' {
                continue;
            }

            let r0c0 = window[0][col];
            let r2c2 = window[2][col + 2];

            if !((r0c0 == 'M' && r2c2 == 'S') || (r0c0 == 'S' && r2c2 == 'M')) {
                continue;
            }

            let r0c2 = window[0][col + 2];
            let r2c0 = window[2][col];

            if !((r0c2 == 'M' && r2c0 == 'S') || (r0c2 == 'S' && r2c0 == 'M')) {
                continue;
            }

            count += 1;
        }
    }

    println!("part2: {count}");
}
