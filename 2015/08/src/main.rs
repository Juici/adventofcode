use std::str::CharIndices;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

struct Entry<'a> {
    src: &'a str,
    value: String,
}

fn main() -> Result<()> {
    let strings = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&strings));
    println!("part 2: {}", part2(&strings));

    Ok(())
}

fn part1(strings: &[Entry]) -> usize {
    strings.iter().map(|entry| entry.src.chars().count() - entry.value.chars().count()).sum()
}

fn part2(strings: &[Entry]) -> usize {
    strings
        .iter()
        .map(|entry| 2 + entry.src.chars().filter(|c| matches!(c, '"' | '\\')).count())
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Entry>> {
    input
        .lines()
        .enumerate()
        .map(|(i, src)| {
            parse_string(src)
                .map(|value| Entry { src, value })
                .with_context(|| format!("invalid string on line {i}"))
        })
        .collect()
}

fn parse_string(src: &str) -> Result<String> {
    let mut chars = src.char_indices();

    anyhow::ensure!(matches!(chars.next(), Some((0, '"'))), "expected `\"` at index 0");

    let mut buf = String::new();

    loop {
        let Some((i, next)) = chars.next() else {
            anyhow::bail!("unterminated string");
        };

        match next {
            '\\' => match chars.next() {
                Some((_, 'x')) => {
                    if let Some(ch) = parse_hex_char(&mut chars) {
                        buf.push(ch);
                    } else {
                        anyhow::bail!("invalid escape at index {i}");
                    }
                }
                Some((_, c @ ('"' | '\\'))) => buf.push(c),
                Some(_) => anyhow::bail!("invalid escape at index {i}"),
                None => anyhow::bail!("unterminated escape at index {i}"),
            },
            '"' => break,
            _ => buf.push(next),
        }
    }

    if let Some((i, next)) = chars.next() {
        anyhow::bail!("unexpected `{next}` at index {i}");
    }

    Ok(buf)
}

fn parse_hex_char(chars: &mut CharIndices) -> Option<char> {
    fn hex(ch: char) -> Option<u8> {
        match ch {
            '0'..='9' => Some(ch as u8 - b'0'),
            'A'..='F' => Some(ch as u8 - b'A' + 10),
            'a'..='f' => Some(ch as u8 - b'a' + 10),
            _ => None,
        }
    }

    let hi = hex(chars.next()?.1)?;
    let lo = hex(chars.next()?.1)?;

    Some(char::from(hi << 4 | lo))
}

#[cfg(test)]
mod example {
    const EXAMPLE: &str = include_str!("./example");

    #[test]
    fn part1() {
        let strings = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(&strings), 12);
    }

    #[test]
    fn part2() {
        let strings = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(&strings), 19);
    }
}
