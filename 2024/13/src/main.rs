#![feature(iter_array_chunks)]

use std::sync::LazyLock;

use anyhow::{Context, Result};
use regex::Regex;

use self::geometry::{Line, Vec2};

mod geometry;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ButtonKind {
    A,
    B,
}

impl ButtonKind {
    fn cost(self) -> i64 {
        match self {
            ButtonKind::A => 3,
            ButtonKind::B => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]

struct Button {
    kind: ButtonKind,
    delta: Vec2,
}

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: Vec2,
}

impl Machine {
    fn solution(&self) -> Option<((i64, i64), i64)> {
        let x_line = Line::from_abc(self.a.delta.x, self.b.delta.x, self.prize.x);
        let y_line = Line::from_abc(self.a.delta.y, self.b.delta.y, self.prize.y);

        let Vec2 { x: a, y: b } = x_line.intersection(&y_line)?;

        let a = a.is_integer().then(|| *a.numer())?;
        let b = b.is_integer().then(|| *b.numer())?;

        let cost = a * self.a.kind.cost() + b * self.b.kind.cost();

        Some(((a, b), cost))
    }
}

fn main() -> Result<()> {
    let machines = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&machines));
    println!("part 2: {}", part2(&machines));

    Ok(())
}

fn part1(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(|m| m.solution()).map(|(_, cost)| cost).sum()
}

fn part2(machines: &[Machine]) -> i64 {
    const OFFSET_SCALAR: i64 = 10000000000000;
    const OFFSET: Vec2 = Vec2 { x: OFFSET_SCALAR, y: OFFSET_SCALAR };

    machines
        .iter()
        .copied()
        .map(|mut m| {
            m.prize += OFFSET;
            m
        })
        .filter_map(|m| m.solution())
        .map(|(_, cost)| cost)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Machine>> {
    input.lines().filter(|line| !line.is_empty()).array_chunks::<3>().map(parse_machine).collect()
}

fn parse_machine([a, b, prize]: [&str; 3]) -> Result<Machine> {
    let a = parse_button(a).with_context(|| format!("invalid button: '{a}'"))?;
    let b = parse_button(b).with_context(|| format!("invalid button: '{b}'"))?;
    let prize = parse_prize(prize).with_context(|| format!("invalid prize: '{prize}'"))?;

    // Sanity check.
    assert_eq!(a.kind, ButtonKind::A);
    assert_eq!(b.kind, ButtonKind::B);

    Ok(Machine { a, b, prize })
}

fn parse_button(button: &str) -> Result<Button> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap());

    let captures = REGEX.captures(button).context("failed to parse button")?;

    let (_, [kind, x, y]) = captures.extract();

    let kind = match kind {
        "A" => ButtonKind::A,
        "B" => ButtonKind::B,
        _ => unreachable!(),
    };

    let x = x.parse().with_context(|| format!("dx overflows: {x}"))?;
    let y = y.parse().with_context(|| format!("dy overflows: {y}"))?;

    Ok(Button { kind, delta: Vec2 { x, y } })
}

fn parse_prize(prize: &str) -> Result<Vec2> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap());

    let captures = REGEX.captures(prize).context("failed to parse prize")?;

    let (_, [x, y]) = captures.extract();

    let x = x.parse().with_context(|| format!("x overflows: {x}"))?;
    let y = y.parse().with_context(|| format!("y overflows: {y}"))?;

    Ok(Vec2 { x, y })
}

#[cfg(test)]
mod example {
    use crate::parse_input;

    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let machines = parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&machines), 480);
    }

    #[test]
    fn part2() {
        let machines = parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&machines), 875318608908);
    }
}
