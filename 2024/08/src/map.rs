use std::ops::Index;
use std::str::FromStr;

use anyhow::{Context, Error, Result};

use crate::vec2::Vec2;

pub trait MapIndex {
    fn get(self, map: &Map) -> Option<&Cell>;
    fn index(self, map: &Map) -> &Cell;
}

impl MapIndex for (usize, usize) {
    fn get(self, map: &Map) -> Option<&Cell> {
        map.grid.get(self.1).and_then(|row| row.get(self.0))
    }

    fn index(self, map: &Map) -> &Cell {
        &map.grid[self.1][self.0]
    }
}

impl MapIndex for Vec2 {
    fn get(self, map: &Map) -> Option<&Cell> {
        let x = usize::try_from(self.x).ok()?;
        let y = usize::try_from(self.y).ok()?;

        (x, y).get(map)
    }

    fn index(self, map: &Map) -> &Cell {
        let Vec2 { x, y } = self;

        if x < 0 || y < 0 {
            panic!("index out of bounds: ({x}, {y})");
        }

        let x = x as usize;
        let y = y as usize;

        (x, y).index(map)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Vec2,
    pub kind: char,
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Node(char),
}

pub struct Map {
    grid: Vec<Vec<Cell>>,
}

impl Map {
    pub fn get<I: MapIndex>(&self, index: I) -> Option<Cell> {
        index.get(self).copied()
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), Cell)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| ((x, y), cell)))
    }

    pub fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.cells().filter_map(|(pos, cell)| match cell {
            Cell::Empty => None,
            Cell::Node(kind) => Some(Node { pos: pos.into(), kind }),
        })
    }
}

impl<I: MapIndex> Index<I> for Map {
    type Output = Cell;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Cell::Empty),
                        '0'..='9' | 'A'..='Z' | 'a'..='z' => Ok(Cell::Node(c)),
                        _ => Err(anyhow::anyhow!("line {i} contains an invalid character: '{c}'")),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()
            .context("failed to parse map")
            .map(|grid| Map { grid })
    }
}
