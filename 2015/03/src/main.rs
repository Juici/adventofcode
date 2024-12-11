use std::collections::HashSet;
use std::mem;

const INPUT: &str = include_str!("./input");

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    houses.insert((x, y));

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => continue,
        }

        houses.insert((x, y));
    }

    houses.len()
}

fn part2(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);

    houses.insert(pos2);

    for c in input.chars() {
        match c {
            '^' => pos1.1 += 1,
            'v' => pos1.1 -= 1,
            '>' => pos1.0 += 1,
            '<' => pos1.0 -= 1,
            _ => continue,
        }

        houses.insert(pos1);

        mem::swap(&mut pos1, &mut pos2);
    }

    houses.len()
}
