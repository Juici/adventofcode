#![feature(iter_map_windows)]

use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

trait Keypad: Copy + Ord + Debug {
    fn neighbors(&self) -> &[Neighbor<Self>];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Neighbor<T> {
    key: T,
    press: ArrowKey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumKey {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Activate,
}

impl NumKey {
    fn digit(self) -> Option<u8> {
        match self {
            Self::Num0 => Some(0),
            Self::Num1 => Some(1),
            Self::Num2 => Some(2),
            Self::Num3 => Some(3),
            Self::Num4 => Some(4),
            Self::Num5 => Some(5),
            Self::Num6 => Some(6),
            Self::Num7 => Some(7),
            Self::Num8 => Some(8),
            Self::Num9 => Some(9),
            Self::Activate => None,
        }
    }
}

impl Keypad for NumKey {
    fn neighbors(&self) -> &[Neighbor<Self>] {
        use ArrowKey as A;
        use NumKey as N;

        match self {
            N::Num0 => &[
                Neighbor { key: N::Num2, press: A::Up },
                Neighbor { key: N::Activate, press: A::Right },
            ],
            N::Num1 => &[
                Neighbor { key: N::Num4, press: A::Up },
                Neighbor { key: N::Num2, press: A::Right },
            ],
            N::Num2 => &[
                Neighbor { key: N::Num5, press: A::Up },
                Neighbor { key: N::Num3, press: A::Right },
                Neighbor { key: N::Num0, press: A::Down },
                Neighbor { key: N::Num1, press: A::Left },
            ],
            N::Num3 => &[
                Neighbor { key: N::Num6, press: A::Up },
                Neighbor { key: N::Activate, press: A::Down },
                Neighbor { key: N::Num2, press: A::Left },
            ],
            N::Num4 => &[
                Neighbor { key: N::Num7, press: A::Up },
                Neighbor { key: N::Num5, press: A::Right },
                Neighbor { key: N::Num1, press: A::Down },
            ],
            N::Num5 => &[
                Neighbor { key: N::Num8, press: A::Up },
                Neighbor { key: N::Num6, press: A::Right },
                Neighbor { key: N::Num2, press: A::Down },
                Neighbor { key: N::Num4, press: A::Left },
            ],
            N::Num6 => &[
                Neighbor { key: N::Num9, press: A::Up },
                Neighbor { key: N::Num3, press: A::Down },
                Neighbor { key: N::Num5, press: A::Left },
            ],
            N::Num7 => &[
                Neighbor { key: N::Num8, press: A::Right },
                Neighbor { key: N::Num4, press: A::Down },
            ],
            N::Num8 => &[
                Neighbor { key: N::Num9, press: A::Right },
                Neighbor { key: N::Num5, press: A::Down },
                Neighbor { key: N::Num7, press: A::Left },
            ],
            N::Num9 => &[
                Neighbor { key: N::Num6, press: A::Down },
                Neighbor { key: N::Num8, press: A::Left },
            ],
            N::Activate => &[
                Neighbor { key: N::Num3, press: A::Up },
                Neighbor { key: N::Num0, press: A::Left },
            ],
        }
    }
}

impl TryFrom<char> for NumKey {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use NumKey::*;

        Ok(match c {
            '0' => Num0,
            '1' => Num1,
            '2' => Num2,
            '3' => Num3,
            '4' => Num4,
            '5' => Num5,
            '6' => Num6,
            '7' => Num7,
            '8' => Num8,
            '9' => Num9,
            'A' => Activate,
            _ => anyhow::bail!("unknown numpad key: '{c}'"),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ArrowKey {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

impl ArrowKey {
    const LIST: [ArrowKey; 5] =
        [ArrowKey::Up, ArrowKey::Right, ArrowKey::Down, ArrowKey::Left, ArrowKey::Activate];
}

impl Keypad for ArrowKey {
    fn neighbors(&self) -> &[Neighbor<Self>] {
        use ArrowKey as A;

        match self {
            A::Up => &[
                Neighbor { key: A::Activate, press: A::Right },
                Neighbor { key: A::Down, press: A::Down },
            ],
            A::Right => &[
                Neighbor { key: A::Activate, press: A::Up },
                Neighbor { key: A::Down, press: A::Left },
            ],
            A::Down => &[
                Neighbor { key: A::Up, press: A::Up },
                Neighbor { key: A::Right, press: A::Right },
                Neighbor { key: A::Left, press: A::Left },
            ],
            A::Left => &[Neighbor { key: A::Down, press: A::Right }],
            A::Activate => &[
                Neighbor { key: A::Right, press: A::Down },
                Neighbor { key: A::Up, press: A::Left },
            ],
        }
    }
}

#[derive(Clone, Debug)]
struct NumSequence(Vec<NumKey>);

impl NumSequence {
    fn to_numeric(&self) -> u64 {
        let mut n = 0;

        for digit in self.0.iter().filter_map(|k| k.digit()) {
            n *= 10;
            n += u64::from(digit);
        }

        n
    }

    fn steps(&self) -> impl Iterator<Item = [NumKey; 2]> + '_ {
        std::iter::once(NumKey::Activate)
            .chain(self.0.iter().copied())
            .map_windows(|&[start, goal]| [start, goal])
    }

    fn cost(&self, costs: &HashMap<[ArrowKey; 2], u64>) -> u64 {
        self.steps().map(|[start, goal]| path_cost(costs, start, goal)).sum()
    }
}

fn build_arrow_costs(level: u32) -> HashMap<[ArrowKey; 2], u64> {
    let mut costs = HashMap::new();

    // Start with uniform cost for level 0.
    for start in ArrowKey::LIST {
        for end in ArrowKey::LIST {
            costs.insert([start, end], 1);
        }
    }

    for _ in 0..level {
        let mut new_costs = HashMap::new();

        for start in ArrowKey::LIST {
            let mut seen = HashSet::new();
            let mut queue = BinaryHeap::new();

            queue.push((Reverse(0), start, ArrowKey::Activate));

            while let Some((Reverse(cost), node, prev)) = queue.pop() {
                if cost > 0 && prev == ArrowKey::Activate {
                    if let Entry::Vacant(entry) = new_costs.entry([start, node]) {
                        entry.insert(cost);
                    }
                } else {
                    queue.push((
                        Reverse(cost + costs[&[prev, ArrowKey::Activate]]),
                        node,
                        ArrowKey::Activate,
                    ));
                }

                seen.insert(node);

                for &neighbor in node.neighbors() {
                    if seen.contains(&neighbor.key) {
                        continue;
                    }

                    queue.push((
                        Reverse(cost + costs[&[prev, neighbor.press]]),
                        neighbor.key,
                        neighbor.press,
                    ));
                }
            }
        }

        costs = new_costs;
    }

    costs
}

fn path_cost<T: Keypad>(costs: &HashMap<[ArrowKey; 2], u64>, start: T, goal: T) -> u64 {
    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), start, ArrowKey::Activate));

    while let Some((Reverse(cost), node, prev)) = queue.pop() {
        if node == goal {
            if prev == ArrowKey::Activate {
                return cost;
            }

            queue.push((
                Reverse(cost + costs[&[prev, ArrowKey::Activate]]),
                node,
                ArrowKey::Activate,
            ));

            continue;
        }

        for neighbor in node.neighbors() {
            queue.push((
                Reverse(cost + costs[&[prev, neighbor.press]]),
                neighbor.key,
                neighbor.press,
            ));
        }
    }

    panic!("no path found from '{start:?}' to '{goal:?}'");
}

fn main() -> Result<()> {
    let input = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[NumSequence]) -> u64 {
    let costs = build_arrow_costs(2);

    input.iter().map(|seq| seq.to_numeric() * seq.cost(&costs)).sum()
}

fn part2(input: &[NumSequence]) -> u64 {
    let costs = build_arrow_costs(25);

    input.iter().map(|seq| seq.to_numeric() * seq.cost(&costs)).sum()
}

fn parse_input(input: &str) -> Result<Vec<NumSequence>> {
    input
        .lines()
        .map(|line| line.chars().map(NumKey::try_from).collect::<Result<Vec<_>>>().map(NumSequence))
        .collect()
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let input = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&input), 126384);
    }
}
