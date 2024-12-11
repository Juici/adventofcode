use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

fn main() -> Result<()> {
    let reports = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&reports));
    println!("part 2: {}", part2(&reports));

    Ok(())
}

fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn part2(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe_dampened()).count()
}

struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn is_safe(&self) -> bool {
        is_safe(self.levels.iter().copied())
    }

    fn is_safe_dampened(&self) -> bool {
        for removed in 0..self.levels.len() {
            let levels =
                self.levels.iter().enumerate().filter(|(i, _)| *i != removed).map(|(_, &v)| v);

            if is_safe(levels) {
                return true;
            }
        }

        false
    }
}

fn is_safe(mut levels: impl Iterator<Item = i64>) -> bool {
    let Some(mut prev) = levels.next() else { return true };

    let mut increasing = None;

    for next in levels {
        match increasing {
            None => {
                increasing = Some(prev < next);
            }
            Some(increasing) if increasing != (prev < next) => return false,
            _ => {}
        }

        let diff: u64 = prev.abs_diff(next);

        if !(1..=3).contains(&diff) {
            return false;
        }

        prev = next;
    }

    true
}

fn parse_input(input: &str) -> Result<Vec<Report>> {
    let mut reports = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut levels = Vec::new();

        for level in line.split_whitespace() {
            let level = level
                .parse::<i64>()
                .with_context(|| format!("failed to parse integer '{level}' on line {i}"))?;

            levels.push(level);
        }

        reports.push(Report { levels });
    }

    Ok(reports)
}
