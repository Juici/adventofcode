use std::ops::{Index, IndexMut};
use std::sync::LazyLock;

use anyhow::{Context, Result};
use regex::Regex;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
struct Instruction {
    command: Command,
    start: (usize, usize),
    end: (usize, usize),
}

impl Instruction {
    fn iter_range(&self) -> impl Iterator<Item = (usize, usize)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        // Instruction ranges are inclusive.
        (x1..=x2).flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
    }
}

#[derive(Clone, Copy, Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Lights {
    grid: [[u8; 1000]; 1000],
}

impl Lights {
    fn iter(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, state)| ((x, y), *state)))
    }
}

impl Default for Lights {
    fn default() -> Self {
        Lights { grid: [[0; 1000]; 1000] }
    }
}

impl Index<(usize, usize)> for Lights {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl IndexMut<(usize, usize)> for Lights {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y][x]
    }
}

fn main() -> Result<()> {
    let instructions = parse_input().context("failed to parse input")?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}

fn part1(instructions: &[Instruction]) {
    let mut lights = Lights::default();

    for instr in instructions {
        for pos in instr.iter_range() {
            let state = &mut lights[pos];

            *state = match instr.command {
                Command::TurnOn => 1,
                Command::TurnOff => 0,
                Command::Toggle if *state == 0 => 1,
                Command::Toggle => 0,
            };
        }
    }

    let lit = lights.iter().filter(|(_, state)| *state != 0).count();

    println!("part1: {lit}");
}

fn part2(instructions: &[Instruction]) {
    let mut lights = Lights::default();

    for instr in instructions {
        for pos in instr.iter_range() {
            let state = &mut lights[pos];

            *state = match instr.command {
                Command::TurnOn => *state + 1,
                Command::TurnOff => state.saturating_sub(1),
                Command::Toggle => *state + 2,
            };
        }
    }

    let brightness = lights.iter().map(|(_, state)| state as u32).sum::<u32>();

    println!("part2: {brightness}");
}

fn parse_input() -> Result<Vec<Instruction>> {
    INPUT
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_instruction(line)
                .with_context(|| format!("failed to parse instruction on line {i}"))
        })
        .collect()
}

fn parse_instruction(instr: &str) -> Result<Instruction> {
    static INSTR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap()
    });

    let captures = INSTR_REGEX.captures(instr).context("invalid instruction")?;

    let (_, [cmd, x1, y1, x2, y2]) = captures.extract();

    let command = match cmd {
        "turn on" => Command::TurnOn,
        "turn off" => Command::TurnOff,
        "toggle" => Command::Toggle,
        _ => unreachable!(),
    };

    let x1 = x1.parse::<usize>()?;
    let y1 = y1.parse::<usize>()?;

    let x2 = x2.parse::<usize>()?;
    let y2 = y2.parse::<usize>()?;

    Ok(Instruction { command, start: (x1, y1), end: (x2, y2) })
}
