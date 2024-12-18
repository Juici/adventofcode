use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::ops::{Add, Index, IndexMut};

use anyhow::{Context, Result};

use self::score::MinScored;

mod score;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn distance(self, other: Vec2) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(isize, isize)> for Vec2 {
    type Output = Option<Vec2>;

    fn add(self, (dx, dy): (isize, isize)) -> Self::Output {
        let x = self.x.checked_add_signed(dx)?;
        let y = self.y.checked_add_signed(dy)?;
        Some(Vec2 { x, y })
    }
}

impl PartialEq<(usize, usize)> for Vec2 {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Safe,
    Corrupted,
}

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        assert_ne!(width, 0, "width should be non-zero");
        assert_ne!(height, 0, "height should be non-zero");

        Grid { width, height, cells: vec![Cell::Safe; width * height] }
    }

    const fn start(&self) -> Vec2 {
        Vec2 { x: 0, y: 0 }
    }

    const fn goal(&self) -> Vec2 {
        Vec2 { x: self.width - 1, y: self.height - 1 }
    }

    const fn try_index(&self, pos: Vec2) -> Option<usize> {
        if pos.x < self.width && pos.y < self.height {
            Some(pos.x + pos.y * self.width)
        } else {
            None
        }
    }

    #[track_caller]
    fn index(&self, pos: Vec2) -> usize {
        match self.try_index(pos) {
            Some(index) => index,
            None => panic!("coordinates out of bounds: {pos}"),
        }
    }

    fn get(&self, pos: Vec2) -> Option<&Cell> {
        self.try_index(pos).and_then(|index| self.cells.get(index))
    }

    fn edges(&self, pos: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        [pos + (0, -1), pos + (1, 0), pos + (0, 1), pos + (-1, 0)]
            .into_iter()
            .flatten()
            .filter(|pos| matches!(self.get(*pos), Some(Cell::Safe)))
    }

    fn shortest_path(&self) -> Option<Vec<Vec2>> {
        let mut queue = BinaryHeap::new();
        let mut estimated_scores = HashMap::new();
        let mut scores = HashMap::new();
        let mut path = HashMap::new();

        let start = self.start();
        let goal = self.goal();

        scores.insert(start, 0);
        queue.push(MinScored(0, start));

        while let Some(MinScored(estimated_score, node)) = queue.pop() {
            let node_score = scores[&node];

            if node == goal {
                let mut node = node;

                let mut path = std::iter::once(node)
                    .chain(std::iter::from_fn(|| {
                        node = path.get(&node).copied()?;

                        Some(node)
                    }))
                    .collect::<Vec<_>>();

                path.reverse();

                return Some(path);
            }

            match estimated_scores.entry(node) {
                Entry::Occupied(entry) => {
                    let score = entry.into_mut();
                    if *score <= estimated_score {
                        // Ignore nodes that have been reached though an equal or shorter path.
                        continue;
                    }
                    *score = estimated_score;
                }
                Entry::Vacant(entry) => {
                    entry.insert(estimated_score);
                }
            }

            for next in self.edges(node) {
                let next_score = node_score + 1;

                match scores.entry(next) {
                    Entry::Occupied(entry) => {
                        let score = entry.into_mut();
                        if *score <= next_score {
                            // Ignore nodes that have been reached though an equal or shorter path.
                            continue;
                        }
                        *score = next_score;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(next_score);
                    }
                }

                let next_estimated_score = next_score + next.distance(goal);

                queue.push(MinScored(next_estimated_score, next));
                path.insert(next, node);
            }
        }

        None
    }
}

impl Index<Vec2> for Grid {
    type Output = Cell;

    fn index(&self, pos: Vec2) -> &Self::Output {
        &self.cells[self.index(pos)]
    }
}

impl IndexMut<Vec2> for Grid {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        let index = self.index(pos);

        &mut self.cells[index]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chunks = self.cells.chunks(self.width);

        let Some(mut chunk) = chunks.next() else { return Ok(()) };

        loop {
            for cell in chunk {
                f.write_str(match cell {
                    Cell::Safe => ".",
                    Cell::Corrupted => "#",
                })?;
            }

            chunk = match chunks.next() {
                Some(chunk) => chunk,
                None => break,
            };

            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let bytes = parse_input(INPUT).context("failed to parse input")?;
    let grid = Grid::new(71, 71);

    println!("part 1: {}", part1(grid.clone(), &bytes, 1024));
    println!("part 2: {}", part2(grid, &bytes, 1024));

    Ok(())
}

fn part1(mut grid: Grid, bytes: &[Vec2], dropped: usize) -> usize {
    let mut bytes = bytes.iter().copied();

    for byte in bytes.by_ref().take(dropped) {
        grid[byte] = Cell::Corrupted;
    }

    grid.shortest_path().and_then(|p| p.len().checked_sub(1)).expect("no path found")
}

fn part2(mut grid: Grid, bytes: &[Vec2], dropped: usize) -> Vec2 {
    let mut bytes = bytes.iter().copied();

    for byte in bytes.by_ref().take(dropped) {
        grid[byte] = Cell::Corrupted;
    }

    let mut path = grid.shortest_path().expect("no path found to begin with");

    for byte in bytes {
        grid[byte] = Cell::Corrupted;

        if !path.contains(&byte) {
            continue;
        }

        path = match grid.shortest_path() {
            Some(path) => path,
            None => return byte,
        };
    }

    panic!("no coordinates found");
}

fn parse_input(input: &str) -> Result<Vec<Vec2>> {
    input
        .lines()
        .map(|pos| {
            let (x, y) = pos.split_once(',').context("missing ',' in coordinate")?;

            let x = x.parse().context("invalid x coordinate")?;
            let y = y.parse().context("invalid y coordinate")?;

            Ok(Vec2 { x, y })
        })
        .collect()
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let bytes = super::parse_input(EXAMPLE).unwrap();
        let grid = super::Grid::new(7, 7);

        assert_eq!(super::part1(grid, &bytes, 12), 22);
    }

    #[test]
    fn part2() {
        let bytes = super::parse_input(EXAMPLE).unwrap();
        let grid = super::Grid::new(7, 7);

        assert_eq!(super::part2(grid, &bytes, 12), (6, 1));
    }
}
