#![feature(array_windows)]

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const LIST: [Direction; 4] =
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(u32, u32);

impl Pos {
    fn neighbor(self, direction: Direction) -> Option<Pos> {
        let Pos(x, y) = self;

        match direction {
            Direction::Up => y.checked_sub(1).map(|y| Pos(x, y)),
            Direction::Right => x.checked_add(1).map(|x| Pos(x, y)),
            Direction::Down => y.checked_add(1).map(|y| Pos(x, y)),
            Direction::Left => x.checked_sub(1).map(|x| Pos(x, y)),
        }
    }

    fn neighbors(self) -> impl Iterator<Item = Pos> {
        Direction::LIST.into_iter().filter_map(move |dir| self.neighbor(dir))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Face {
    pos: Pos,
    dir: Direction,
}

struct Cell {
    pos: Pos,
    plant: char,
}

struct Map {
    cells: Vec<Vec<char>>,
}

impl Map {
    fn get(&self, pos: Pos) -> Option<char> {
        let x = usize::try_from(pos.0).ok()?;
        let y = usize::try_from(pos.1).ok()?;

        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Cell> + '_ {
        pos.neighbors().filter_map(|pos| self.get(pos).map(|plant| Cell { pos, plant }))
    }

    fn plants(&self) -> impl Iterator<Item = Cell> + '_ {
        (0u32..).zip(self.cells.iter()).flat_map(|(y, row)| {
            (0u32..).zip(row.iter()).map(move |(x, &plant)| Cell { pos: Pos(x, y), plant })
        })
    }
}

impl Map {
    fn from_str(input: &str) -> Map {
        let grid = input.lines().map(|line| line.chars().collect()).collect();

        Map { cells: grid }
    }
}

struct Region {
    plant: char,
    cells: HashSet<Pos>,
}

impl Region {
    fn new(plant: char) -> Region {
        Region { plant, cells: HashSet::new() }
    }

    fn insert(&mut self, pos: Pos) -> bool {
        self.cells.insert(pos)
    }

    fn area(&self) -> usize {
        self.cells.len()
    }

    // Unsorted edge faces of the region.
    fn edges(&self) -> impl Iterator<Item = Face> + '_ {
        self.cells.iter().flat_map(|pos| {
            Direction::LIST
                .into_iter()
                .filter(|&dir| match pos.neighbor(dir) {
                    Some(cell) => !self.cells.contains(&cell),
                    None => true,
                })
                .map(|dir| Face { pos: *pos, dir })
        })
    }
}

fn main() {
    let map = Map::from_str(INPUT);

    let regions = find_regions(&map);

    println!("part 1: {}", part1(&regions));
    println!("part 2: {}", part2(&regions));
}

fn part1(regions: &[Region]) -> usize {
    regions.iter().map(|region| region.area() * region.edges().count()).sum()
}

fn part2(regions: &[Region]) -> usize {
    fn count_distinct_faces(map: HashMap<u32, Vec<u32>>) -> usize {
        map.into_values()
            .map(|mut line| {
                line.sort_unstable();

                let mut count = line.len();

                for &[a, b] in line.array_windows::<2>() {
                    if a.abs_diff(b) == 1 {
                        count -= 1;
                    }
                }

                count
            })
            .sum()
    }

    regions
        .iter()
        .map(|region| {
            let mut up = HashMap::<u32, Vec<u32>>::new();
            let mut right = HashMap::<u32, Vec<u32>>::new();
            let mut down = HashMap::<u32, Vec<u32>>::new();
            let mut left = HashMap::<u32, Vec<u32>>::new();

            for Face { pos, dir } in region.edges() {
                match dir {
                    Direction::Up => up.entry(pos.1).or_default().push(pos.0),
                    Direction::Right => right.entry(pos.0).or_default().push(pos.1),
                    Direction::Down => down.entry(pos.1).or_default().push(pos.0),
                    Direction::Left => left.entry(pos.0).or_default().push(pos.1),
                }
            }

            let faces = count_distinct_faces(up)
                + count_distinct_faces(right)
                + count_distinct_faces(down)
                + count_distinct_faces(left);

            region.area() * faces
        })
        .sum()
}

fn find_regions(map: &Map) -> Vec<Region> {
    let mut regions = Vec::<Region>::new();
    let mut region_map = HashMap::<Pos, usize>::new();

    let mut stack = Vec::new();

    for cell in map.plants() {
        if region_map.contains_key(&cell.pos) {
            continue;
        }

        let region_id = regions.len();

        regions.push(Region::new(cell.plant));

        let region = &mut regions[region_id];

        stack.clear();
        stack.push(cell);

        while let Some(cell) = stack.pop() {
            region.insert(cell.pos);
            region_map.insert(cell.pos, region_id);

            for cell in map.neighbors(cell.pos) {
                if cell.plant == region.plant && !region_map.contains_key(&cell.pos) {
                    stack.push(cell);
                }
            }
        }
    }

    regions
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let map = super::Map::from_str(EXAMPLE);
        let regions = super::find_regions(&map);

        assert_eq!(super::part1(&regions), 1930);
    }

    #[test]
    fn part2() {
        let map = super::Map::from_str(EXAMPLE);
        let regions = super::find_regions(&map);

        assert_eq!(super::part2(&regions), 1206);
    }
}
