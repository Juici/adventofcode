use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

use anyhow::{Context, Error, Result};

use self::geometry::{Direction, Vec2};
use self::score::MinScored;

mod geometry;
mod score;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pos: Vec2,
    dir: Direction,
}

impl Node {
    fn from_start(start: Vec2) -> Node {
        Node { pos: start, dir: Direction::East }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: HashMap<Vec2, Tile>,
    start: Node,
    goal: Vec2,
}

impl Map {
    fn edges(&self, node: &Node) -> impl Iterator<Item = (i32, Node)> {
        let forward = node.pos.adjacent(node.dir);

        matches!(self.tiles.get(&forward), Some(Tile::Floor))
            .then(|| (1, forward, node.dir))
            .into_iter()
            .chain([
                (1000, node.pos, node.dir.turn_left()),
                (1000, node.pos, node.dir.turn_right()),
            ])
            .map(|(cost, pos, dir)| (cost, Node { pos, dir }))
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut tiles = HashMap::new();

        for (y, line) in (0i32..).zip(input.lines()) {
            for (x, tile) in (0i32..).zip(line.chars()) {
                let pos = Vec2 { x, y };
                let tile = match tile {
                    '.' => Tile::Floor,
                    '#' => Tile::Wall,
                    'S' => {
                        if start.replace(pos).is_some() {
                            anyhow::bail!("found more than one start");
                        }
                        Tile::Floor
                    }
                    'E' => {
                        if end.replace(pos).is_some() {
                            anyhow::bail!("found more than one end");
                        }
                        Tile::Floor
                    }
                    _ => anyhow::bail!("unexpected tile: '{tile}'"),
                };

                tiles.insert(pos, tile);
            }
        }

        let start = start.context("no start found")?;
        let end = end.context("no end found")?;

        let start = Node::from_start(start);

        Ok(Map { tiles, start, goal: end })
    }
}

fn main() -> Result<()> {
    let map = INPUT.parse::<Map>().context("failed to parse input")?;

    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &Map) -> i32 {
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    costs.insert(map.start, 0);
    queue.push(MinScored(0, map.start));

    while let Some(MinScored(node_cost, node)) = queue.pop() {
        if node.pos == map.goal {
            return node_cost;
        }

        for (cost, next) in map.edges(&node) {
            let next_cost = node_cost + cost;

            match costs.entry(next) {
                Entry::Occupied(entry) => {
                    let cost = entry.into_mut();
                    if *cost <= next_cost {
                        // Ignore nodes that have been reached though a shorter path.
                        continue;
                    }
                    *cost = next_cost;
                }
                Entry::Vacant(entry) => {
                    entry.insert(next_cost);
                }
            }

            queue.push(MinScored(next_cost, next));
        }
    }

    panic!("no path found");
}

fn part2(map: &Map) -> usize {
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut paths = HashMap::new();

    costs.insert(map.start, 0);
    queue.push(MinScored(0, map.start));

    let mut ends = HashSet::new();

    while let Some(MinScored(node_cost, node)) = queue.pop() {
        if node.pos == map.goal {
            let min_cost = node_cost;

            while let Some(MinScored(cost, node)) = queue.pop() {
                if cost != min_cost {
                    break;
                }
                if node.pos == map.goal {
                    ends.insert(node);
                }
            }

            break;
        }

        for (cost, next) in map.edges(&node) {
            let next_cost = node_cost + cost;

            match costs.entry(next) {
                Entry::Occupied(entry) => {
                    let cost = entry.into_mut();
                    match next_cost.cmp(cost) {
                        // Ignore nodes that have been reached though a shorter path.
                        Ordering::Greater => continue,
                        Ordering::Less => {
                            paths.insert(next, HashSet::from([node]));
                        }
                        Ordering::Equal => {
                            paths.entry(next).or_default().insert(node);
                        }
                    }
                    *cost = next_cost;
                }
                Entry::Vacant(entry) => {
                    entry.insert(next_cost);
                    paths.insert(next, HashSet::from([node]));
                }
            }

            queue.push(MinScored(next_cost, next));
        }
    }

    let mut stack = Vec::from_iter(ends);
    let mut seen = HashSet::new();

    while let Some(node) = stack.pop() {
        if !seen.insert(node) {
            continue;
        }

        if let Some(prev) = paths.get(&node) {
            stack.extend(prev);
        }
    }

    seen.into_iter().map(|node| node.pos).collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod example1 {
    const EXAMPLE: &str = include_str!("./example1");

    #[test]
    fn part1() {
        let map = EXAMPLE.parse::<super::Map>().unwrap();

        assert_eq!(super::part1(&map), 7036);
    }

    #[test]
    fn part2() {
        let map = EXAMPLE.parse::<super::Map>().unwrap();

        assert_eq!(super::part2(&map), 45);
    }
}

#[cfg(test)]
mod example2 {
    const EXAMPLE: &str = include_str!("./example2");

    #[test]
    fn part1() {
        let map = EXAMPLE.parse::<super::Map>().unwrap();

        assert_eq!(super::part1(&map), 11048);
    }

    #[test]
    fn part2() {
        let map = EXAMPLE.parse::<super::Map>().unwrap();

        assert_eq!(super::part2(&map), 64);
    }
}
