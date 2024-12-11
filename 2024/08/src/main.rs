use std::collections::{HashMap, HashSet};

use anyhow::Result;

use self::map::Map;
use self::vec2::Vec2;

mod map;
mod vec2;

const INPUT: &str = include_str!("./input");

fn main() -> Result<()> {
    let map = INPUT.parse::<Map>()?;

    part1(&map);
    part2(&map);

    Ok(())
}

fn part1(map: &Map) {
    let mut nodes = HashMap::<char, HashSet<Vec2>>::new();

    for node in map.nodes() {
        nodes.entry(node.kind).or_default().insert(node.pos);
    }

    let mut antinodes = HashSet::new();

    for nodes in nodes.values() {
        for &start in nodes {
            for &end in nodes {
                if start == end {
                    continue;
                }

                let diff = end - start;

                let anti1 = start - diff;
                let anti2 = end + diff;

                if map.get(anti1).is_some() {
                    antinodes.insert(anti1);
                }
                if map.get(anti2).is_some() {
                    antinodes.insert(anti2);
                }
            }
        }
    }

    println!("part1: {}", antinodes.len());
}

fn part2(map: &Map) {
    let mut nodes = HashMap::<char, HashSet<Vec2>>::new();

    for node in map.nodes() {
        nodes.entry(node.kind).or_default().insert(node.pos);
    }

    let mut antinodes = HashSet::new();

    for nodes in nodes.values() {
        for &start in nodes {
            for &end in nodes {
                if start == end {
                    continue;
                }

                let diff = end - start;

                {
                    let mut anti = start;
                    while map.get(anti).is_some() {
                        antinodes.insert(anti);
                        anti -= diff;
                    }
                }

                {
                    let mut anti = end;
                    while map.get(anti).is_some() {
                        antinodes.insert(anti);
                        anti += diff;
                    }
                }
            }
        }
    }

    println!("part2: {}", antinodes.len());
}
