use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use self::geometry::{Direction, Vec2};

mod geometry;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Box,
    Wall,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: HashMap<Vec2, Tile>,
    robot: Vec2,
}

impl Map {
    fn move_robot(&mut self, dir: Direction) -> bool {
        let space = self.robot.adjacent(dir);

        // Fast path checks for robot moving without touching boxes.
        match self.tiles.get(&space) {
            None | Some(Tile::Wall) => return false,
            Some(Tile::Floor) => {
                self.robot = space;
                return true;
            }
            Some(Tile::Box) => {}
        }

        // The robot will attempt to push at least one box.
        let mut end_space = space;

        loop {
            end_space = end_space.adjacent(dir);

            match self.tiles.get(&end_space) {
                None | Some(Tile::Wall) => return false,
                Some(Tile::Floor) => break,
                Some(Tile::Box) => continue,
            }
        }

        // Update robot position.
        self.robot = space;

        // Apply updates to pushed boxes.
        // We can ignore tiles in between the new spaces of the first and last box.
        self.tiles.insert(space, Tile::Floor);
        self.tiles.insert(end_space, Tile::Box);

        true
    }

    fn to_map2(&self) -> Map2 {
        fn scale(v: &Vec2) -> Vec2 {
            Vec2 { x: v.x * 2, y: v.y }
        }

        let mut tiles = HashMap::new();

        for (pos, tile) in &self.tiles {
            let lpos = scale(pos);
            let rpos = lpos.adjacent(Direction::Right);

            let (left, right) = match tile {
                Tile::Floor => (Tile2::Floor, Tile2::Floor),
                Tile::Box => (Tile2::BoxLeft, Tile2::BoxRight),
                Tile::Wall => (Tile2::Wall, Tile2::Wall),
            };

            tiles.insert(lpos, left);
            tiles.insert(rpos, right);
        }

        let robot = scale(&self.robot);

        Map2 { tiles, robot }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile2 {
    Floor,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Clone, Debug)]
struct Map2 {
    tiles: HashMap<Vec2, Tile2>,
    robot: Vec2,
}

impl Map2 {
    fn move_robot(&mut self, dir: Direction) -> bool {
        let new_pos = self.robot.adjacent(dir);

        let mut stack = Vec::new();

        // Fast path checks for robot moving without touching boxes.
        match self.tiles.get(&new_pos) {
            None | Some(Tile2::Wall) => return false,
            Some(Tile2::Floor) => {
                self.robot = new_pos;
                return true;
            }
            Some(Tile2::BoxLeft) => {
                stack.push((new_pos, Tile2::BoxLeft));
                stack.push((new_pos.adjacent(Direction::Right), Tile2::BoxRight));
            }
            Some(Tile2::BoxRight) => {
                stack.push((new_pos, Tile2::BoxRight));
                stack.push((new_pos.adjacent(Direction::Left), Tile2::BoxLeft));
            }
        }

        let mut seen = HashSet::new();
        let mut boxes = Vec::new();

        // Find and check all boxes that will be moved.
        while let Some((pos, tile)) = stack.pop() {
            if seen.contains(&pos) {
                continue;
            }

            let new_pos = pos.adjacent(dir);

            match self.tiles.get(&new_pos) {
                None | Some(Tile2::Wall) => return false,
                Some(Tile2::Floor) => {}
                Some(Tile2::BoxLeft) => {
                    stack.push((new_pos, Tile2::BoxLeft));
                    stack.push((new_pos.adjacent(Direction::Right), Tile2::BoxRight));
                }
                Some(Tile2::BoxRight) => {
                    stack.push((new_pos, Tile2::BoxRight));
                    stack.push((new_pos.adjacent(Direction::Left), Tile2::BoxLeft));
                }
            }

            seen.insert(pos);
            boxes.push((pos, tile));
        }

        // Update robot position.
        self.robot = new_pos;

        // Update boxes.
        for (pos, _) in &mut boxes {
            // Remove the box from the tile map.
            self.tiles.insert(*pos, Tile2::Floor);

            // Get the new position of the box.
            *pos += dir.vector::<i32>();
        }
        for (pos, tile) in boxes {
            // Insert the box back into the map at its new position.
            self.tiles.insert(pos, tile);
        }

        true
    }
}

fn main() -> Result<()> {
    let (map, moves) = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&map, &moves));
    println!("part 2: {}", part2(&map, &moves));

    Ok(())
}

fn part1(map: &Map, moves: &[Direction]) -> i32 {
    let mut map = map.clone();

    for &dir in moves {
        map.move_robot(dir);
    }

    map.tiles
        .into_iter()
        .filter(|(_, t)| matches!(t, Tile::Box))
        .map(|(p, _)| p.x + (p.y * 100))
        .sum()
}

fn part2(map: &Map, moves: &[Direction]) -> i32 {
    let mut map = map.to_map2();

    for &dir in moves {
        map.move_robot(dir);
    }

    map.tiles
        .into_iter()
        .filter(|(_, t)| matches!(t, Tile2::BoxLeft))
        .map(|(p, _)| p.x + (p.y * 100))
        .sum()
}

fn parse_input(input: &str) -> Result<(Map, Vec<Direction>)> {
    let mut robot = None;
    let mut tiles = HashMap::new();

    let (map_input, moves_input) =
        input.split_once("\n\n").context("failed to split input into map and moves")?;

    for (y, line) in (0i32..).zip(map_input.lines()) {
        for (x, tile) in (0i32..).zip(line.chars()) {
            let pos = Vec2 { x, y };
            let tile = match tile {
                '.' => Tile::Floor,
                'O' => Tile::Box,
                '#' => Tile::Wall,
                '@' => {
                    if robot.replace(pos).is_some() {
                        anyhow::bail!("found more than one robot");
                    }
                    Tile::Floor
                }
                _ => anyhow::bail!("unexpected tile: '{tile}'"),
            };

            tiles.insert(pos, tile);
        }
    }

    let robot = robot.context("no robot found in map")?;
    let map = Map { tiles, robot };

    let moves = moves_input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => anyhow::bail!("invalid move: '{c}'"),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((map, moves))
}

#[cfg(test)]
mod small_example {

    const EXAMPLE: &str = include_str!("./small_example");

    #[test]
    fn part1() {
        let (map, moves) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&map, &moves), 2028);
    }
}

#[cfg(test)]
mod big_example {

    const EXAMPLE: &str = include_str!("./big_example");

    #[test]
    fn part1() {
        let (map, moves) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&map, &moves), 10092);
    }

    #[test]
    fn part2() {
        let (map, moves) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&map, &moves), 9021);
    }
}
