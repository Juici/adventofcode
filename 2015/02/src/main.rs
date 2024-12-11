use std::mem;

use anyhow::{Context, Error, Result};

const INPUT: &str = include_str!("./input");

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

fn main() -> Result<()> {
    let boxes = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&boxes));
    println!("part 2: {}", part2(&boxes));

    Ok(())
}

fn part1(boxes: &[Box]) -> u32 {
    let mut sum = 0;

    for Box { l, w, h } in boxes {
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let smallest = lw.min(wh).min(hl);

        sum += 2 * lw + 2 * wh + 2 * hl + smallest;
    }

    sum
}

fn part2(boxes: &[Box]) -> u32 {
    let mut sum = 0;

    for Box { l, w, h } in boxes {
        let mut a = l;
        let mut b = w;
        let mut c = h;

        if a > b {
            mem::swap(&mut a, &mut b);
        }
        if b > c {
            mem::swap(&mut b, &mut c);
        }

        let perim = a + a + b + b;
        let vol = l * w * h;

        sum += perim + vol;
    }

    sum
}

fn parse_input(input: &str) -> Result<Vec<Box>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('x');

            let l = split.next().context("missing length")?;
            let w = split.next().context("missing width")?;
            let h = split.next().context("missing height")?;

            let l = l.parse::<u32>().context("failed to parse length")?;
            let w = w.parse::<u32>().context("failed to parse width")?;
            let h = h.parse::<u32>().context("failed to parse height")?;

            Ok::<Box, Error>(Box { l, w, h })
        })
        .enumerate()
        .map(|(i, result)| result.with_context(|| format!("failed to parse line {i}")))
        .collect()
}
