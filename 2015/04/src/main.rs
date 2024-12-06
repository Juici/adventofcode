use md5::{Digest, Md5};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

const INPUT: &str = "iwrupvqb";

fn main() {
    part1();
    part2();
}

fn part1() {
    let n = (1..u32::MAX).into_par_iter().by_exponential_blocks().find_first(|&n| {
        let mut hasher = Md5::new();

        hasher.update(INPUT);
        hasher.update(itoa::Buffer::new().format(n));

        let digest = hasher.finalize();
        let digest = digest.as_slice();

        matches!(digest, [0, 0, 0x0..0x10, ..])
    });

    match n {
        Some(n) => println!("part1: {n}"),
        None => println!("part1: not found"),
    }
}

fn part2() {
    let n = (1..u32::MAX).into_par_iter().by_exponential_blocks().find_first(|&n| {
        let mut hasher = Md5::new();

        hasher.update(INPUT);
        hasher.update(itoa::Buffer::new().format(n));

        let digest = hasher.finalize();
        let digest = digest.as_slice();

        matches!(digest, [0, 0, 0, ..])
    });

    match n {
        Some(n) => println!("part2: {n}"),
        None => println!("part2: not found"),
    }
}
