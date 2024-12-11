use std::collections::HashMap;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

fn main() -> Result<()> {
    let (mut left, mut right) = parse_input(INPUT).context("failed to parse input")?;

    left.sort_unstable();
    right.sort_unstable();

    println!("part 1: {}", part1(&left, &right));
    println!("part 2: {}", part2(&left, &right));

    Ok(())
}

fn part1(left: &[i32], right: &[i32]) -> u32 {
    left.iter().zip(right).map(|(&a, &b)| a.abs_diff(b)).sum()
}

fn part2(left: &[i32], right: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &v in right {
        counts.entry(v).and_modify(|n| *n += 1).or_insert(1);
    }

    left.iter().filter_map(|v| counts.get(v).map(|count| v * count)).sum()
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let (a, b) = line
            .trim()
            .split_once(' ')
            .with_context(|| format!("invalid format for line {i}: '{line}'"))?;

        let a = a.trim();
        let b = b.trim();

        let a = a
            .parse::<i32>()
            .with_context(|| format!("failed to parse integer '{a}' on line {i}"))?;
        let b = b
            .parse::<i32>()
            .with_context(|| format!("failed to parse integer '{b}' on line {i}"))?;

        left.push(a);
        right.push(b);
    }

    Ok((left, right))
}
