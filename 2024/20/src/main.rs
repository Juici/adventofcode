use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use self::score::MinScored;

mod score;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn manhattan_distance(self, other: Vec2) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn manhattan_neighbors(self, r: usize) -> impl Iterator<Item = Vec2> {
        (0..=r).flat_map(move |dx| {
            (0..=(r - dx))
                .flat_map(move |dy| {
                    fn v(x: Option<usize>, y: Option<usize>) -> Option<Vec2> {
                        Some(Vec2 { x: x?, y: y? })
                    }

                    let x1 = self.x.checked_sub(dx);
                    let x2 = self.x.checked_add(dx);
                    let y1 = self.y.checked_sub(dy);
                    let y2 = self.y.checked_add(dy);

                    let v1 = v(x1, y1);
                    let v2 = v(x1, y2);
                    let v3 = v(x2, y1);
                    let v4 = v(x2, y2);

                    [v1, v2, v3, v4].into_iter()
                })
                .flatten()
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn get(&self, pos: Vec2) -> Option<Tile> {
        self.tiles.get(pos.y).and_then(|row| row.get(pos.x)).copied()
    }

    fn manhattan_neighbors(&self, pos: Vec2, r: usize) -> impl Iterator<Item = Vec2> + '_ {
        pos.manhattan_neighbors(r).filter(|pos| self.get(*pos).is_some())
    }

    fn edges(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        pos.manhattan_neighbors(1)
            .filter_map(|pos| self.get(pos).map(|tile| (pos, tile)))
            .filter(|(_, tile)| matches!(tile, Tile::Floor))
            .map(|(pos, _)| pos)
    }

    fn dijkstra(
        &self,
        start: Vec2,
        goal: Vec2,
        limit: Option<usize>,
    ) -> (HashMap<Vec2, usize>, HashMap<Vec2, Vec<Vec2>>) {
        let mut queue = BinaryHeap::new();
        let mut scores = HashMap::new();
        let mut parents = HashMap::new();
        let mut seen = HashSet::new();

        scores.insert(start, 0);
        queue.push(MinScored(0, start));

        while let Some(MinScored(node_score, node)) = queue.pop() {
            if !seen.insert(node) {
                continue;
            }

            if node == goal || limit == Some(node_score) {
                break;
            }

            for next in self.edges(node) {
                if seen.contains(&next) {
                    continue;
                }

                let next_score = node_score + 1;

                match scores.entry(next) {
                    Entry::Vacant(entry) => {
                        entry.insert(next_score);
                        parents.insert(next, vec![node]);
                    }
                    Entry::Occupied(entry) => {
                        let score = entry.into_mut();

                        match next_score.cmp(score) {
                            // Ignore nodes that have been reached though a shorter path.
                            Ordering::Greater => continue,
                            Ordering::Equal => {
                                parents.entry(next).or_default().push(node);
                            }
                            Ordering::Less => {
                                parents.insert(next, vec![node]);
                                *score = next_score;
                            }
                        }
                    }
                }

                queue.push(MinScored(next_score, next));
            }
        }

        (scores, parents)
    }
}

fn main() -> Result<()> {
    let input = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> usize {
    cheats(input, 2).into_values().filter(|save| *save >= 100).count()
}

fn part2(input: &Input) -> usize {
    cheats(input, 20).into_values().filter(|save| *save >= 100).count()
}

fn cheats(input: &Input, cheat_len: usize) -> HashMap<(Vec2, Vec2), usize> {
    let Input { ref map, start, goal } = *input;

    let (scores, parents) = map.dijkstra(start, goal, None);

    let cheat_starts = {
        let mut nodes = HashSet::from([goal]);
        let mut stack = parents[&goal].clone();

        while let Some(node) = stack.pop() {
            if !nodes.insert(node) {
                continue;
            }

            if let Some(parents) = parents.get(&node) {
                stack.extend(parents);
            }
        }

        nodes
    };

    cheat_starts
        .into_par_iter()
        .flat_map_iter(|cheat_start| {
            map.manhattan_neighbors(cheat_start, cheat_len)
                .map(move |cheat_end| (cheat_start, cheat_end))
        })
        .filter_map(|(cheat_start, cheat_end)| {
            let start_score = scores[&cheat_start];
            let end_score = scores.get(&cheat_end).copied()?;

            let distance = cheat_start.manhattan_distance(cheat_end);
            let save = end_score.checked_sub(start_score)?.checked_sub(distance)?;

            Some(((cheat_start, cheat_end), save))
        })
        .collect()
}

struct Input {
    map: Map,
    start: Vec2,
    goal: Vec2,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut start = None;
    let mut goal = None;

    let mut tiles = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, tile) in line.chars().enumerate() {
            let pos = Vec2 { x, y };
            let tile = match tile {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                'S' => {
                    if start.replace(pos).is_some() {
                        anyhow::bail!("more than one start found");
                    }
                    Tile::Floor
                }
                'E' => {
                    if goal.replace(pos).is_some() {
                        anyhow::bail!("more than one goal found");
                    }
                    Tile::Floor
                }
                _ => anyhow::bail!("unknown tile character: '{tile}'"),
            };

            row.push(tile);
        }

        tiles.push(row);
    }

    let start = start.context("no start found")?;
    let goal = goal.context("no goal found")?;
    let map = Map { tiles };

    Ok(Input { map, start, goal })
}

#[cfg(test)]
mod example {
    use std::collections::HashMap;
    use std::hash::Hash;

    const EXAMPLE: &str = include_str!("./example");

    fn count_values<K, V: Eq + Hash>(map: HashMap<K, V>) -> HashMap<V, usize> {
        let mut counts = HashMap::new();

        for value in map.into_values() {
            counts.entry(value).and_modify(|n| *n += 1).or_insert(1);
        }

        counts
    }

    #[test]
    fn part1() {
        let input = super::parse_input(EXAMPLE).unwrap();
        let cheats = super::cheats(&input, 2);

        let counts = count_values(cheats);

        let expect = |save, count: usize| match counts.get(&save) {
            Some(n) => assert_eq!(*n, count),
            None => panic!("no cheats save {save} ps"),
        };

        expect(2, 14);
        expect(4, 14);
        expect(6, 2);
        expect(8, 4);
        expect(10, 2);
        expect(12, 3);
        expect(20, 1);
        expect(36, 1);
        expect(38, 1);
        expect(40, 1);
        expect(64, 1);
    }

    #[test]
    fn part2() {
        let input = super::parse_input(EXAMPLE).unwrap();
        let cheats = super::cheats(&input, 20);

        let counts = count_values(cheats);

        let expect = |save, count: usize| match counts.get(&save) {
            Some(n) => assert_eq!(*n, count),
            None => panic!("no cheats save {save} ps"),
        };

        expect(50, 32);
        expect(52, 31);
        expect(54, 29);
        expect(56, 39);
        expect(58, 25);
        expect(60, 23);
        expect(62, 20);
        expect(64, 19);
        expect(66, 12);
        expect(68, 14);
        expect(70, 12);
        expect(72, 22);
        expect(74, 4);
        expect(76, 3);
    }
}
