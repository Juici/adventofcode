use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

use anyhow::{Context, Error, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for Color {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Color::White),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            _ => Err(anyhow::anyhow!("unknown color: '{c}'")),
        }
    }
}

#[derive(Clone, Debug)]
struct Pattern(Vec<Color>);

impl Pattern {
    fn is_prefix_to(&self, colors: &[Color]) -> bool {
        let prefix = &self.0[..];
        let n = prefix.len();

        n <= colors.len() && prefix == &colors[..n]
    }
}

#[derive(Clone, Debug)]
struct Design(Vec<Color>);

impl Design {
    fn solutions(&self, patterns: &[Pattern]) -> u64 {
        let mut queue = BinaryHeap::new();
        let mut counts = HashMap::new();

        counts.insert(0, 1);
        queue.push(Reverse(0));

        while let Some(Reverse(offset)) = queue.pop() {
            let remaining = &self.0[offset..];
            let count = counts[&offset];

            if remaining.is_empty() {
                return count;
            }

            for next_offset in
                patterns.iter().filter(|p| p.is_prefix_to(remaining)).map(|p| offset + p.0.len())
            {
                match counts.entry(next_offset) {
                    Entry::Occupied(entry) => {
                        *entry.into_mut() += count;
                        continue;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(count);
                    }
                }

                queue.push(Reverse(next_offset));
            }
        }

        0
    }
}

fn main() -> Result<()> {
    let (patterns, designs) = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&patterns, &designs));
    println!("part 2: {}", part2(&patterns, &designs));

    Ok(())
}

fn part1(patterns: &[Pattern], designs: &[Design]) -> usize {
    designs.par_iter().filter(|design| design.solutions(patterns) > 0).count()
}

fn part2(patterns: &[Pattern], designs: &[Design]) -> u64 {
    designs.par_iter().map(|design| design.solutions(patterns)).sum()
}

fn parse_input(input: &str) -> Result<(Vec<Pattern>, Vec<Design>)> {
    let (patterns, designs) =
        input.split_once("\n\n").context("cannot split input into patterns and designs")?;

    let patterns = patterns
        .split(',')
        .map(|s| s.trim().chars().map(Color::try_from).collect::<Result<Vec<Color>>>().map(Pattern))
        .collect::<Result<Vec<_>>>()?;

    let designs = designs
        .lines()
        .map(|s| s.trim().chars().map(Color::try_from).collect::<Result<Vec<Color>>>().map(Design))
        .collect::<Result<Vec<_>>>()?;

    Ok((patterns, designs))
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let (patterns, designs) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&patterns, &designs), 6);
    }

    #[test]
    fn part2() {
        let (patterns, designs) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&patterns, &designs), 16);
    }
}
