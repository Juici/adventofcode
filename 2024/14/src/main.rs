#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{Context, Error, Result};
use num::Zero;
use regex::Regex;

use self::geometry::Vec2;

mod geometry;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_int(n: &str) -> Result<i32> {
            n.parse().with_context(|| format!("failed to parse int: '{n}'"))
        }

        static REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"p=([\+-]?\d+),([\+-]?\d+) v=([\+-]?\d+),([\+-]?\d+)").unwrap()
        });

        let captures = REGEX.captures(s).context("failed to parse robot")?;

        let (_, [px, py, vx, vy]) = captures.extract();

        Ok(Robot {
            position: Vec2 { x: parse_int(px)?, y: parse_int(py)? },
            velocity: Vec2 { x: parse_int(vx)?, y: parse_int(vy)? },
        })
    }
}

impl Robot {
    fn step(&mut self, map: &Map) {
        self.position += self.velocity;

        // Wrap the robot if it has left the map.
        self.position.x = self.position.x.rem_euclid(map.width);
        self.position.y = self.position.y.rem_euclid(map.height);
    }
}

#[derive(Clone, Copy, Debug)]
struct Map {
    width: i32,
    height: i32,
}

impl Map {
    fn quadrants(&self) -> [Quadrant; 4] {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        let l_start = 0;
        let l_end = half_width;

        let t_start = 0;
        let t_end = half_height;

        let r_start = self.width - half_width;
        let r_end = self.width;

        let b_start = self.height - half_height;
        let b_end = self.height;

        [
            Quadrant { start: Vec2 { x: l_start, y: t_start }, end: Vec2 { x: l_end, y: t_end } },
            Quadrant { start: Vec2 { x: r_start, y: t_start }, end: Vec2 { x: r_end, y: t_end } },
            Quadrant { start: Vec2 { x: r_start, y: b_start }, end: Vec2 { x: r_end, y: b_end } },
            Quadrant { start: Vec2 { x: l_start, y: b_start }, end: Vec2 { x: l_end, y: b_end } },
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct Quadrant {
    start: Vec2,
    end: Vec2,
}

impl Quadrant {
    fn contains(&self, pos: &Vec2) -> bool {
        let x_range = self.start.x..self.end.x;
        let y_range = self.start.y..self.end.y;

        x_range.contains(&pos.x) && y_range.contains(&pos.y)
    }
}

fn main() -> Result<()> {
    let robots = parse_input(INPUT).context("failed to parse input")?;
    let map = Map { width: 101, height: 103 };

    println!("part 1: {}", part1(&robots, &map));
    println!("part 2: {}", part2(&robots, &map));

    Ok(())
}

fn part1(robots: &[Robot], map: &Map) -> i64 {
    let mut robots = robots.to_vec();

    for _ in 0..100 {
        for robot in &mut robots {
            robot.step(map);
        }
    }

    let quadrants = map.quadrants();
    let mut counts = [0; 4];

    'robots: for robot in robots {
        for (quad, count) in quadrants.iter().zip(counts.iter_mut()) {
            if quad.contains(&robot.position) {
                *count += 1;
                continue 'robots;
            }
        }
    }

    counts.into_iter().product()
}

fn part2(robots: &[Robot], map: &Map) -> i32 {
    let mut robots = robots.to_vec();

    // let mut stdin = std::io::stdin().lines();

    for i in 0.. {
        {
            let positions = robots.iter().map(|r| r.position).collect::<HashSet<Vec2>>();

            // Use a rudimentary density function to find likely candidates.
            let density = positions
                .iter()
                .map(|pos| {
                    (-1..1)
                        .flat_map(|dx| (-1..1).map(move |dy| Vec2 { x: dx, y: dy }))
                        .filter(|v| !v.is_zero())
                        .map(|v| *pos + v)
                        .filter(|p| positions.contains(p))
                        .count()
                })
                .sum::<usize>();

            if density > 200 {
                // print_map(&robots, map);
                // println!("seconds = {i}");

                // if stdin.next().is_none() {
                //     break;
                // }

                return i;
            }
        }

        for robot in &mut robots {
            robot.step(map);
        }
    }

    panic!("not found");
}

// fn print_map(robots: &[Robot], map: &Map) {
//     let width = usize::try_from(map.width).unwrap();
//     let height = usize::try_from(map.height).unwrap();

//     let mut grid = vec![0; width * height];

//     for Robot { position, .. } in robots {
//         let Ok(index) = usize::try_from(position.y * map.width + position.x) else {
//             continue;
//         };

//         grid[index] += 1;
//     }

//     println!();

//     for line in grid.chunks(width) {
//         for &cell in line {
//             if cell > 0 {
//                 print!("{cell}");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(Robot::from_str).collect()
}

#[cfg(test)]
mod example {

    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let robots = super::parse_input(EXAMPLE).unwrap();
        let map = super::Map { width: 11, height: 7 };

        assert_eq!(super::part1(&robots, &map), 12);
    }
}
