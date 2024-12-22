use std::collections::hash_map::Entry;
use std::collections::HashMap;

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
struct Prng {
    state: u32,
}

impl Prng {
    const fn from_seed(seed: u32) -> Prng {
        Prng { state: seed }
    }

    const fn advance(&mut self) -> u32 {
        const MASK: u32 = (1 << 24) - 1;

        const fn mix(state: u32, value: u32) -> u32 {
            state ^ value
        }

        const fn prune(value: u32) -> u32 {
            value & MASK
        }

        self.state = prune(mix(self.state, self.state << 6));
        self.state = prune(mix(self.state, self.state >> 5));
        self.state = prune(mix(self.state, self.state << 11));

        self.state
    }

    const fn next(self) -> Prng {
        let mut next = self;
        next.advance();
        next
    }

    const fn price(self) -> u8 {
        (self.state % 10) as u8
    }

    const fn change_from(&self, prev: &Prng) -> i8 {
        let prev = prev.price() as i8;
        let currect = self.price() as i8;

        currect - prev
    }

    const fn start_history<const N: usize>(self) -> History<N> {
        History::start(self)
    }
}

#[derive(Clone, Copy, Debug)]
struct History<const N: usize> {
    history: [i8; N],
    prev: Prng,
}

impl<const N: usize> History<N> {
    const fn start(prng: Prng) -> History<N> {
        let mut history = [0; N];

        let mut prev = prng;
        let mut index = 0;

        while index < N {
            let next = prev.next();

            history[index] = next.change_from(&prev);

            index += 1;
            prev = next;
        }

        History { history, prev }
    }

    const fn price(&self) -> u8 {
        self.prev.price()
    }

    const fn history(&self) -> [i8; N] {
        self.history
    }

    const fn len(&self) -> usize {
        N
    }

    fn advance(&mut self) -> u8 {
        let prev = self.prev;
        let next = prev.next();

        self.history.copy_within(1.., 0);
        self.history[N - 1] = next.change_from(&prev);

        self.prev = next;

        self.price()
    }
}

fn main() -> Result<()> {
    let input = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Prng]) -> u64 {
    input
        .par_iter()
        .copied()
        .map(|mut prng| {
            for _ in 0..2000 {
                prng.advance();
            }
            prng.state
        })
        .map(u64::from)
        .sum()
}

fn part2(input: &[Prng]) -> u64 {
    input
        .par_iter()
        .map(|start| {
            let mut prices = HashMap::new();
            let mut history = start.start_history::<4>();

            for _ in history.len()..2000 {
                // Only count the first appearance of a sequence.
                if let Entry::Vacant(entry) = prices.entry(history.history()) {
                    entry.insert(u64::from(history.price()));
                }

                history.advance();
            }

            prices
        })
        .reduce_with(|mut a, b| {
            for (history, price) in b {
                a.entry(history).and_modify(|v| *v += price).or_insert(price);
            }
            a
        })
        .and_then(|prices| prices.into_values().max())
        .expect("no price history")
}

fn parse_input(input: &str) -> Result<Vec<Prng>> {
    input
        .lines()
        .map(|n| {
            n.parse().map(Prng::from_seed).with_context(|| format!("invalid prng seed: '{n}'"))
        })
        .collect()
}

#[cfg(test)]
mod example {
    const EXAMPLE1: &str = include_str!("./example1");
    const EXAMPLE2: &str = include_str!("./example2");

    #[test]
    fn prng() {
        let mut prng = super::Prng { state: 123 };

        assert_eq!(prng.advance(), 15887950);
        assert_eq!(prng.advance(), 16495136);
        assert_eq!(prng.advance(), 527345);
        assert_eq!(prng.advance(), 704524);
        assert_eq!(prng.advance(), 1553684);
        assert_eq!(prng.advance(), 12683156);
        assert_eq!(prng.advance(), 11100544);
        assert_eq!(prng.advance(), 12249484);
        assert_eq!(prng.advance(), 7753432);
        assert_eq!(prng.advance(), 5908254);
    }

    #[test]
    fn part1() {
        let input = super::parse_input(EXAMPLE1).unwrap();

        assert_eq!(super::part1(&input), 37327623);
    }

    #[test]
    fn part2() {
        let input = super::parse_input(EXAMPLE2).unwrap();

        assert_eq!(super::part2(&input), 23);
    }
}
