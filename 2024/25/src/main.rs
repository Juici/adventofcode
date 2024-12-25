use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Lock([u8; 5]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Key([u8; 5]);

impl Key {
    fn check(&self, lock: &Lock) -> bool {
        self.0.into_iter().zip(lock.0).all(|(a, b)| a + b <= 5)
    }
}

fn main() -> Result<()> {
    let (locks, keys) = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&locks, &keys));
    println!("part 2 is free");

    Ok(())
}

fn part1(locks: &[Lock], keys: &[Key]) -> usize {
    keys.iter().map(|key| locks.iter().filter(|lock| key.check(lock)).count()).sum()
}

fn parse_input(input: &str) -> Result<(Vec<Lock>, Vec<Key>)> {
    enum LockOrKey {
        Lock(Lock),
        Key(Key),
    }

    fn parse_lock_or_key(chunk: &str) -> Option<LockOrKey> {
        const PINS: usize = 5;
        const HEIGHT: usize = 5;

        let mut heights = [0; PINS];
        let mut lines = chunk.lines();

        let top = lines.next()?;

        for row in lines.by_ref().take(HEIGHT) {
            if row.len() > PINS {
                return None;
            }

            for (i, c) in row.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => heights[i] += 1,
                    _ => return None,
                }
            }
        }

        let bottom = lines.next()?;

        if lines.next().is_some() {
            return None;
        }

        if top == "#####" && bottom == "....." {
            Some(LockOrKey::Lock(Lock(heights)))
        } else if top == "....." && bottom == "#####" {
            Some(LockOrKey::Key(Key(heights)))
        } else {
            None
        }
    }

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for chunk in input.split("\n\n") {
        match parse_lock_or_key(chunk) {
            Some(LockOrKey::Lock(lock)) => locks.push(lock),
            Some(LockOrKey::Key(key)) => keys.push(key),
            None => anyhow::bail!("invalid lock or key:\n\n{chunk}"),
        }
    }

    Ok((locks, keys))
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let (locks, keys) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&locks, &keys), 3);
    }
}
