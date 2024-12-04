use std::collections::hash_map::Entry;

use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("./input");

fn main() -> Result<()> {
    let (mut left, mut right) = parse_input().context("failed to parse input")?;

    left.sort();
    right.sort();

    part1(&left, &right);
    part2(&left, &right);

    Ok(())
}

fn part1(left: &[i64], right: &[i64]) {
    let mut distance = 0;

    for (&a, &b) in left.iter().zip(right) {
        distance += a.abs_diff(b);
    }

    println!("part 1: {distance}");
}

fn part2(left: &[i64], right: &[i64]) {
    let mut counts = FxHashMap::default();

    for &v in right {
        match counts.entry(v) {
            Entry::Occupied(entry) => {
                *entry.into_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    let mut similarity = 0;

    for &v in left {
        if let Some(&count) = counts.get(&v) {
            similarity += v * count;
        }
    }

    println!("part 2: {similarity}");
}

fn parse_input() -> Result<(Vec<i64>, Vec<i64>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (i, line) in INPUT.lines().enumerate() {
        let (a, b) = line
            .trim()
            .split_once(' ')
            .with_context(|| format!("invalid format for line {i}: '{line}'"))?;

        let a = a.trim();
        let b = b.trim();

        let a = a
            .parse::<i64>()
            .with_context(|| format!("failed to parse integer '{a}' on line {i}"))?;
        let b = b
            .parse::<i64>()
            .with_context(|| format!("failed to parse integer '{b}' on line {i}"))?;

        left.push(a);
        right.push(b);
    }

    Ok((left, right))
}
