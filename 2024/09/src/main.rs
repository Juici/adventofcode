use anyhow::{Context, Result};

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
enum Block {
    File { id: usize },
    Empty,
}

impl Block {
    fn id(self) -> Option<usize> {
        match self {
            Block::File { id } => Some(id),
            Block::Empty => None,
        }
    }
}

fn main() -> Result<()> {
    let blocks = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(&blocks));
    println!("part 2: {}", part2(&blocks));

    Ok(())
}

fn part1(blocks: &[Block]) -> usize {
    let mut blocks = blocks.to_vec();

    let mut start = 0;

    'compact: loop {
        'empty: loop {
            match blocks.get(start) {
                Some(Block::Empty) => break 'empty,
                Some(Block::File { .. }) => start += 1,
                None => break 'compact,
            }
        }

        let last = loop {
            match blocks.pop() {
                Some(b @ Block::File { .. }) => break b,
                Some(Block::Empty) => continue,
                None => break 'compact,
            };
        };

        if start >= blocks.len() {
            break;
        }

        blocks[start] = last;
        start += 1;
    }

    checksum(&blocks)
}

fn part2(blocks: &[Block]) -> usize {
    let mut blocks = blocks.to_vec();

    let files = {
        let mut files = Vec::new();
        let mut iter = blocks.iter().enumerate().peekable();

        while let Some((start, b)) = iter.next() {
            let cur_id = match b {
                Block::File { id } => *id,
                Block::Empty => continue,
            };

            let end = loop {
                match iter.peek() {
                    Some((_, Block::File { id })) if *id == cur_id => {
                        iter.next();
                    }
                    Some((i, _)) => break *i,
                    None => break blocks.len(),
                }
            };

            files.push(start..end);
        }

        files
    };

    'compact: for file in files.into_iter().rev() {
        let (haystack, tail) = blocks.split_at_mut(file.start);

        let file = &mut tail[..file.len()];

        let empty = {
            let mut start = 0;

            let empty = loop {
                loop {
                    match haystack.get(start) {
                        Some(Block::File { .. }) => start += 1,
                        Some(Block::Empty) => break,
                        None => continue 'compact,
                    }
                }

                let mut end = start + 1;

                while let Some(Block::Empty) = haystack.get(end) {
                    end += 1;
                }

                let empty = start..end;
                if file.len() <= empty.len() {
                    break empty;
                }

                start = end;
            };

            &mut haystack[empty]
        };

        empty[..file.len()].swap_with_slice(file);
    }

    checksum(&blocks)
}

fn checksum(blocks: &[Block]) -> usize {
    blocks.iter().enumerate().filter_map(|(pos, b)| b.id().map(|id| pos * id)).sum()
}

fn parse_input(input: &str) -> Result<Vec<Block>> {
    let input = input.trim();

    let mut id = 0;
    let mut file = true;

    let mut blocks = Vec::with_capacity(input.len());

    for (i, c) in input.chars().enumerate() {
        let size =
            c.to_digit(10).with_context(|| format!("invalid character '{c}' at position {i}"))?
                as usize;

        let block: Block;

        if file {
            block = Block::File { id };

            id += 1;
            file = false;
        } else {
            block = Block::Empty;

            file = true;
        }

        for _ in 0..size {
            blocks.push(block);
        }
    }

    Ok(blocks)
}
