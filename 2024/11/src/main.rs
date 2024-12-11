use std::collections::HashMap;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

fn main() -> Result<()> {
    let stones = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&stones));
    println!("part 2: {}", part2(&stones));

    Ok(())
}

fn part1(stones: &[u64]) -> usize {
    let mut stones = build_stone_map(stones);

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.values().sum()
}

fn part2(stones: &[u64]) -> usize {
    let mut stones = build_stone_map(stones);

    for _ in 0..75 {
        stones = blink(stones);
    }

    stones.values().sum()
}

fn build_stone_map(stones: &[u64]) -> HashMap<u64, usize> {
    let mut map = HashMap::default();

    for &stone in stones {
        add_stones(&mut map, stone, 1);
    }

    map
}

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones = HashMap::default();

    for (stone, count) in stones {
        let digits = match stone {
            0 => {
                add_stones(&mut new_stones, 1, count);
                continue;
            }
            n => n.ilog10() + 1,
        };

        if digits % 2 != 0 {
            add_stones(&mut new_stones, stone * 2024, count);
            continue;
        }

        let split = digits / 2;
        let divisor = 10u64.pow(split);

        let left = stone / divisor;
        let right = stone % divisor;

        add_stones(&mut new_stones, left, count);
        add_stones(&mut new_stones, right, count);
    }

    new_stones
}

fn add_stones(stones: &mut HashMap<u64, usize>, stone: u64, count: usize) {
    stones.entry(stone).and_modify(|v| *v += count).or_insert(count);
}

fn parse_input(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u64>().with_context(|| format!("invalid stone: {n}")))
        .collect::<Result<Vec<_>>>()
}
