use std::collections::HashMap;
use std::hash::BuildHasher;

use anyhow::{Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("./input");

#[derive(Debug, Clone, Copy)]
enum Cell {
    Floor,
    Obstruction,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up = 1 << 0,
    Right = 1 << 1,
    Down = 1 << 2,
    Left = 1 << 3,
}

impl Direction {
    fn apply(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => y.checked_sub(1).map(|y| (x, y)),
            Direction::Right => x.checked_add(1).map(|x| (x, y)),
            Direction::Down => y.checked_add(1).map(|y| (x, y)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    /// Runs the guard through the map.
    ///
    /// Returns true if the guard encountered a loop.
    fn run<M, S>(
        &mut self,
        map: &M,
        coverage: &mut HashMap<(usize, usize), DirectionSet, S>,
    ) -> bool
    where
        M: Mappable,
        S: BuildHasher,
    {
        loop {
            // Add cell to covered path, breaking from loop if the guard has already covered it in that direction.
            if !coverage.entry(self.position).or_default().insert(self.direction) {
                break true;
            }

            // Step for guard, breaking from loop if the guard leaves the map.
            if !self.step(map) {
                break false;
            }
        }
    }

    fn step<M>(&mut self, map: &M) -> bool
    where
        M: Mappable,
    {
        match self
            .direction
            .apply(self.position)
            .and_then(|pos| map.get(pos).map(|cell| (pos, cell)))
        {
            Some((pos, cell)) => {
                match cell {
                    Cell::Floor => self.position = pos,
                    Cell::Obstruction => self.direction = self.direction.turn_right(),
                }
                true
            }
            None => false,
        }
    }
}

trait Mappable {
    fn get(&self, pos: (usize, usize)) -> Option<Cell>;
}

#[derive(Debug, Clone)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone)]
struct LayeredMap<'a> {
    map: &'a Map,
    obstruction: (usize, usize),
}

#[derive(Debug, Default)]
struct DirectionSet {
    bits: u8,
}

impl DirectionSet {
    fn insert(&mut self, direction: Direction) -> bool {
        let direction = direction as u8;

        if self.bits & direction == 0 {
            self.bits |= direction;
            true
        } else {
            false
        }
    }
}

impl Mappable for Map {
    fn get(&self, (x, y): (usize, usize)) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }
}

impl Map {
    fn iter(&self) -> impl Iterator<Item = ((usize, usize), Cell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter().copied().enumerate().map(move |(x, cell)| ((x, y), cell))
        })
    }
}

impl Mappable for LayeredMap<'_> {
    fn get(&self, pos: (usize, usize)) -> Option<Cell> {
        if pos == self.obstruction {
            Some(Cell::Obstruction)
        } else {
            self.map.get(pos)
        }
    }
}

fn main() -> Result<()> {
    let (map, guard) = parse_input()?;

    part1(&map, guard);
    part2(&map, guard);

    Ok(())
}

fn part1(map: &Map, mut guard: Guard) {
    let mut coverage = FxHashMap::<(usize, usize), DirectionSet>::default();

    guard.run(map, &mut coverage);

    println!("part1: {}", coverage.len());
}

fn part2(map: &Map, guard: Guard) {
    let loops = map
        .iter()
        .filter(|(_, cell)| matches!(cell, Cell::Floor))
        .par_bridge()
        .filter(|&(pos, _)| {
            let mut coverage = FxHashMap::<(usize, usize), DirectionSet>::default();
            let mut guard = guard;

            // Add a single obstruction to the map.
            let map = LayeredMap { map, obstruction: pos };

            guard.run(&map, &mut coverage)
        })
        .count();

    println!("part2: {loops}");
}

fn parse_input() -> Result<(Map, Guard)> {
    let mut cells = Vec::new();
    let mut guard = None;

    for (y, line) in INPUT.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => Cell::Floor,
                '#' => Cell::Obstruction,
                '^' => {
                    guard = Some(Guard { position: (x, y), direction: Direction::Up });
                    Cell::Floor
                }
                _ => anyhow::bail!("invalid map character at ({x}, {y}): '{c}'"),
            };
            row.push(cell);
        }

        cells.push(row);
    }

    Ok((Map { cells }, guard.context("no guard found")?))
}
