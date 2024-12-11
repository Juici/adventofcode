use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

struct Equation {
    result: u64,
    inputs: Vec<u64>,
}

impl Equation {
    fn apply(&self, ops: &[Operation]) -> u64 {
        let mut inputs = self.inputs.iter().copied();
        let mut value = inputs.next().expect("no inputs to equation");

        if inputs.len() != ops.len() {
            panic!("incorrect number of operations for equation, expected: {}", inputs.len());
        }

        for (next, op) in inputs.zip(ops) {
            value = match op {
                Operation::Add => value + next,
                Operation::Multiply => value * next,
                Operation::Concat => {
                    let digits = match next {
                        0 => 1,
                        n => n.ilog10() + 1,
                    };

                    value * 10u64.pow(digits) + next
                }
            };
        }

        value
    }
}

fn main() -> Result<()> {
    let equations = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&equations));
    println!("part 2: {}", part2(&equations));

    Ok(())
}

fn part1(equations: &[Equation]) -> u64 {
    const OPS: [Operation; 2] = [Operation::Add, Operation::Multiply];

    equations
        .par_iter()
        .filter(|equation| {
            let n_ops = match equation.inputs.len().checked_sub(1) {
                Some(n) => n,
                None => return equation.inputs.first().copied() == Some(equation.result),
            };

            std::iter::repeat_with(|| OPS.into_iter())
                .take(n_ops)
                .multi_cartesian_product()
                .par_bridge()
                .any(|ops| equation.apply(&ops) == equation.result)
        })
        .map(|equation| equation.result)
        .sum()
}

fn part2(equations: &[Equation]) -> u64 {
    const OPS: [Operation; 3] = [Operation::Add, Operation::Multiply, Operation::Concat];

    equations
        .par_iter()
        .filter(|equation| {
            let n_ops = match equation.inputs.len().checked_sub(1) {
                Some(n) => n,
                None => return equation.inputs.first().copied() == Some(equation.result),
            };

            std::iter::repeat_with(|| OPS.into_iter())
                .take(n_ops)
                .multi_cartesian_product()
                .par_bridge()
                .any(|ops| equation.apply(&ops) == equation.result)
        })
        .map(|equation| equation.result)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Equation>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_equation(line).with_context(|| format!("invalid equation on line {i}"))
        })
        .collect()
}

fn parse_equation(s: &str) -> Result<Equation> {
    let (result, inputs) = s.split_once(':').context("missing ':'")?;

    let result = result.parse::<u64>().with_context(|| format!("invalid result: '{result}'"))?;
    let inputs = inputs
        .split_whitespace()
        .map(|v| v.parse::<u64>().with_context(|| format!("invalid input: '{v}'")))
        .collect::<Result<Vec<_>>>()?;

    Ok(Equation { result, inputs })
}
