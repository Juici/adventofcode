use md5::{Digest, Md5};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

const INPUT: &str = "iwrupvqb";

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    (1..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&n| {
            let mut hasher = Md5::new();

            hasher.update(input);
            hasher.update(itoa::Buffer::new().format(n));

            let digest = hasher.finalize();
            let digest = digest.as_slice();

            matches!(digest, [0, 0, 0x0..0x10, ..])
        })
        .expect("not found")
}

fn part2(input: &str) -> u32 {
    (1..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&n| {
            let mut hasher = Md5::new();

            hasher.update(input);
            hasher.update(itoa::Buffer::new().format(n));

            let digest = hasher.finalize();
            let digest = digest.as_slice();

            matches!(digest, [0, 0, 0, ..])
        })
        .expect("not found")
}
