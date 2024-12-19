use std::collections::{HashMap, VecDeque};
use std::ops::Index;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use regex::Regex;

const INPUT: &str = include_str!("./input");

type Signal = u16;
type Wire = str;

#[derive(Clone, Debug, Default)]
struct Circuit<'a> {
    wires: HashMap<&'a Wire, Signal>,
}

impl<'a> Circuit<'a> {
    fn set(&mut self, wire: &'a Wire, signal: Signal) {
        self.wires.insert(wire, signal);
    }

    fn get(&self, wire: &'_ Wire) -> Option<&Signal> {
        self.wires.get(wire)
    }
}

impl Index<&'_ Wire> for Circuit<'_> {
    type Output = Signal;

    fn index(&self, wire: &'_ Wire) -> &Self::Output {
        match self.get(wire) {
            Some(signal) => signal,
            None => panic!("circuit does not contain signal for wire: '{wire}'"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Source<'a> {
    Const(Signal),
    Wire(&'a Wire),
}

impl Source<'_> {
    fn try_eval(self, circuit: &Circuit) -> Option<Signal> {
        match self {
            Source::Const(signal) => Some(signal),
            Source::Wire(wire) => circuit.get(wire).copied(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate<'a> {
    Ident(Source<'a>),
    Not(Source<'a>),
    And { lhs: Source<'a>, rhs: Source<'a> },
    Or { lhs: Source<'a>, rhs: Source<'a> },
    Shl { lhs: Source<'a>, rhs: Source<'a> },
    Shr { lhs: Source<'a>, rhs: Source<'a> },
}

impl Gate<'_> {
    fn try_eval(self, circuit: &Circuit) -> Option<Signal> {
        match self {
            Gate::Ident(input) => input.try_eval(circuit),
            Gate::Not(input) => input.try_eval(circuit).map(|v| !v),
            Gate::And { lhs, rhs } => Some(lhs.try_eval(circuit)? & rhs.try_eval(circuit)?),
            Gate::Or { lhs, rhs } => Some(lhs.try_eval(circuit)? | rhs.try_eval(circuit)?),
            Gate::Shl { lhs, rhs } => Some(lhs.try_eval(circuit)? << rhs.try_eval(circuit)?),
            Gate::Shr { lhs, rhs } => Some(lhs.try_eval(circuit)? >> rhs.try_eval(circuit)?),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction<'a> {
    gate: Gate<'a>,
    output: &'a Wire,
}

fn main() -> Result<()> {
    let instructions = parse_input(INPUT).context("failed to parse input")?;

    let a = part1(&instructions);

    println!("part 1: {a}");
    println!("part 2: {}", part2(&instructions, a));

    Ok(())
}

fn part1(instructions: &[Instruction]) -> Signal {
    emulate(instructions.to_vec().into())["a"]
}

fn part2(instructions: &[Instruction], a: Signal) -> Signal {
    let mut instructions = VecDeque::from(instructions.to_vec());

    for instr in &mut instructions {
        if instr.output == "b" {
            instr.gate = Gate::Ident(Source::Const(a));
        }
    }

    emulate(instructions)["a"]
}

fn emulate(mut instructions: VecDeque<Instruction>) -> Circuit {
    let mut circuit = Circuit::default();

    while let Some(instr) = instructions.pop_front() {
        if let Some(result) = instr.gate.try_eval(&circuit) {
            circuit.set(instr.output, result);
        } else {
            instructions.push_back(instr);
        }
    }

    circuit
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Result<Instruction> {
    static CONST_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+) -> ([a-z]+)$").unwrap());
    static NOT_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^NOT (\d+|[a-z]+) -> ([a-z]+)$").unwrap());
    static AND_OR_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+) (AND|OR) (\d+|[a-z]+) -> ([a-z]+)$").unwrap());
    static SHRL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(\d+|[a-z]+) ([LR]SHIFT) (\d+|[a-z]+) -> ([a-z]+)$").unwrap()
    });

    if let Some(cap) = CONST_REGEX.captures(line) {
        let (_, [input, output]) = cap.extract();

        let input = input.parse().map(Source::Const).unwrap_or(Source::Wire(input));

        return Ok(Instruction { gate: Gate::Ident(input), output });
    }

    if let Some(cap) = NOT_REGEX.captures(line) {
        let (_, [input, output]) = cap.extract();

        let input = input.parse().map(Source::Const).unwrap_or(Source::Wire(input));

        return Ok(Instruction { gate: Gate::Not(input), output });
    }

    if let Some(cap) = AND_OR_REGEX.captures(line) {
        let (_, [lhs, gate, rhs, output]) = cap.extract();

        let lhs = lhs.parse().map(Source::Const).unwrap_or(Source::Wire(lhs));
        let rhs = rhs.parse().map(Source::Const).unwrap_or(Source::Wire(rhs));

        let gate = match gate {
            "AND" => Gate::And { lhs, rhs },
            "OR" => Gate::Or { lhs, rhs },
            _ => unreachable!(),
        };

        return Ok(Instruction { gate, output });
    }

    if let Some(cap) = SHRL_REGEX.captures(line) {
        let (_, [lhs, gate, rhs, output]) = cap.extract();

        let lhs = lhs.parse().map(Source::Const).unwrap_or(Source::Wire(lhs));
        let rhs = rhs.parse().map(Source::Const).unwrap_or(Source::Wire(rhs));

        let gate = match gate {
            "LSHIFT" => Gate::Shl { lhs, rhs },
            "RSHIFT" => Gate::Shr { lhs, rhs },
            _ => unreachable!(),
        };

        return Ok(Instruction { gate, output });
    }

    Err(anyhow::anyhow!("invalid instruction: '{line}'"))
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn emulate() {
        let instructions = super::parse_input(EXAMPLE).unwrap();
        let wires = super::emulate(instructions.to_vec().into());

        assert_eq!(wires["d"], 72);
        assert_eq!(wires["e"], 507);
        assert_eq!(wires["f"], 492);
        assert_eq!(wires["g"], 114);
        assert_eq!(wires["h"], 65412);
        assert_eq!(wires["i"], 65079);
        assert_eq!(wires["x"], 123);
        assert_eq!(wires["y"], 456);
    }
}
