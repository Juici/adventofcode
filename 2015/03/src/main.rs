use std::collections::HashSet;
use std::mem;

const INPUT: &str = include_str!("./input");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    houses.insert((x, y));

    for c in INPUT.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => continue,
        }

        houses.insert((x, y));
    }

    println!("part1: {}", houses.len());
}

fn part2() {
    let mut houses = HashSet::new();

    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);

    houses.insert(pos2);

    for c in INPUT.chars() {
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

    println!("part2: {}", houses.len());
}
