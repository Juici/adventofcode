#![feature(ascii_char)]

use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::str::FromStr;
use std::{ascii, fmt};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wire([ascii::Char; 3]);

impl FromStr for Wire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_ascii()
            .and_then(|s| s.try_into().ok())
            .map(Wire)
            .with_context(|| format!("invalid wire name: '{s}'"))
    }
}

impl Wire {
    fn parse_input(s: &str) -> Result<(Wire, bool)> {
        let (wire, value) = s.split_once(": ").with_context(|| format!("invalid wire: '{s}'"))?;

        let wire = wire.parse()?;

        let value = match value {
            "1" => true,
            "0" => false,
            _ => anyhow::bail!("invalid wire value: '{value}'"),
        };

        Ok((wire, value))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for Wire {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        str::fmt(self, f)
    }
}

impl PartialEq<&'_ str> for Wire {
    fn eq(&self, other: &&'_ str) -> bool {
        self.as_str() == *other
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GateOp {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Gate {
    input1: Wire,
    input2: Wire,
    output: Wire,
    op: GateOp,
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let input1 = parts.next().context("missing input 1")?;
        let op = parts.next().context("missing gate")?;
        let input2 = parts.next().context("missing input 2")?;

        anyhow::ensure!(parts.next() == Some("->"), "invalid connection: '{s}'");

        let output = parts.next().context("missing output")?;

        let input1 = input1.parse()?;
        let input2 = input2.parse()?;
        let output = output.parse()?;

        let op = match op {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => anyhow::bail!("invalid gate: '{op}'"),
        };

        Ok(Gate { input1, input2, output, op })
    }
}

impl Gate {
    fn has_input(&self, wire: Wire) -> bool {
        self.input1 == wire || self.input2 == wire
    }

    fn try_eval(&self, circuit: &Circuit) -> Option<bool> {
        let input1 = circuit.get(&self.input1)?;
        let input2 = circuit.get(&self.input2)?;

        Some(match self.op {
            GateOp::And => input1 && input2,
            GateOp::Or => input1 || input2,
            GateOp::Xor => input1 != input2,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct Circuit {
    wires: HashMap<Wire, bool>,
}

impl Circuit {
    fn set(&mut self, wire: Wire, signal: bool) {
        self.wires.insert(wire, signal);
    }

    fn get(&self, wire: &Wire) -> Option<bool> {
        self.wires.get(wire).copied()
    }

    fn emulate(&mut self, gates: &[Gate]) -> u64 {
        let mut queue = VecDeque::from(gates.to_vec());

        while let Some(gate) = queue.pop_front() {
            match gate.try_eval(self) {
                Some(result) => self.set(gate.output, result),
                None => queue.push_back(gate),
            }
        }

        self.get_int('z')
    }

    fn get_int(&self, ch: char) -> u64 {
        let mut n = 0;

        for (wire, value) in &self.wires {
            let Some(shift) = wire.strip_prefix(ch) else { continue };
            let Ok(shift) = shift.parse::<u32>() else { continue };

            n |= (*value as u64) << shift;
        }

        n
    }
}

fn main() -> Result<()> {
    let (inputs, gates) = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&inputs, &gates));
    println!("part 2: {}", part2(&inputs, &gates));

    Ok(())
}

fn part1(inputs: &HashMap<Wire, bool>, gates: &[Gate]) -> u64 {
    let mut circuit = Circuit { wires: inputs.clone() };

    circuit.emulate(gates)
}

fn part2(_inputs: &HashMap<Wire, bool>, gates: &[Gate]) -> String {
    let highest_z =
        gates.iter().filter_map(|g| g.output.strip_prefix('z')).max().expect("no z wires found");

    let mut wrong = Vec::new();

    for gate in gates {
        if gate.op != GateOp::Xor
            && matches!(gate.output.strip_prefix('z'), Some(n) if n != highest_z)
        {
            wrong.push(gate.output);
            continue;
        }

        match gate.op {
            GateOp::Xor => {
                if !matches!(gate.output.0[0].to_char(), 'x' | 'y' | 'z')
                    && !matches!(gate.input1.0[0].to_char(), 'x' | 'y' | 'z')
                    && !matches!(gate.input2.0[0].to_char(), 'x' | 'y' | 'z')
                {
                    wrong.push(gate.output);
                    continue;
                }

                if gates.iter().any(|g| g.has_input(gate.output) && g.op == GateOp::Or) {
                    wrong.push(gate.output);
                }
            }
            GateOp::And => {
                if gate.input1 != "x00"
                    && gate.input2 != "x00"
                    && gates.iter().any(|g| g.has_input(gate.output) && g.op != GateOp::Or)
                {
                    wrong.push(gate.output);
                }
            }
            _ => {}
        }
    }

    wrong.sort_unstable();

    let mut wrong = wrong.into_iter();

    match wrong.next() {
        None => String::new(),
        Some(first) => {
            let mut s = first.as_str().to_owned();

            for next in wrong {
                s.push(',');
                s.push_str(next.as_str());
            }

            s
        }
    }
}

fn parse_input(input: &str) -> Result<(HashMap<Wire, bool>, Vec<Gate>)> {
    let (initial, connections) = input
        .split_once("\n\n")
        .context("failed to split input into initial signals and connections")?;

    let inputs = initial.lines().map(Wire::parse_input).collect::<Result<HashMap<_, _>>>()?;
    let gates = connections.lines().map(Gate::from_str).collect::<Result<Vec<_>>>()?;

    Ok((inputs, gates))
}

#[cfg(test)]
mod example1 {
    const EXAMPLE: &str = include_str!("./example1");

    #[test]
    fn part1() {
        let (inputs, connections) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&inputs, &connections), 4);
    }
}

#[cfg(test)]
mod example2 {
    const EXAMPLE: &str = include_str!("./example2");
    const RESULTS: &str = include_str!("./results2");

    #[test]
    fn expected() {
        let (inputs, connections) = super::parse_input(EXAMPLE).unwrap();

        let expected = RESULTS
            .lines()
            .map(super::Wire::parse_input)
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap();

        let mut circuit = super::Circuit { wires: inputs };

        circuit.emulate(&connections);

        for (wire, expected) in expected {
            let actual = circuit.get(&wire);

            if actual != Some(expected) {
                let bit_repr = |b: bool| if b { "1" } else { "0" };

                panic!(
                    "{wire}: expected {}, but found {}",
                    bit_repr(expected),
                    actual.map_or("nothing", bit_repr),
                );
            }
        }
    }

    #[test]
    fn part1() {
        let (inputs, connections) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&inputs, &connections), 2024);
    }
}

#[cfg(test)]
mod example3 {
    const EXAMPLE: &str = include_str!("./example3");

    #[test]
    #[ignore = "part2 solution is specifically tailored to the question input"]
    fn part2() {
        let (inputs, connections) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&inputs, &connections), "z00,z01,z02,z05");
    }
}
