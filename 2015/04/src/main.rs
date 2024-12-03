use md5::digest::Output;
use md5::{Digest, Md5};

const INPUT: &str = "iwrupvqb";

fn main() {
    part1();
    part2();
}

fn part1() {
    let hasher = Md5::new_with_prefix(INPUT);

    let mut buf = itoa::Buffer::new();
    let mut n = 1;

    let mut digest = Output::<Md5>::default();

    loop {
        let mut hasher = hasher.clone();

        hasher.update(buf.format(n));
        hasher.finalize_into(&mut digest);

        let digest = digest.as_slice();

        if matches!(digest, [0, 0, 0x0..0x10, ..]) {
            println!("part1: {n}");
            return;
        }

        if n == i32::MAX {
            println!("part1: not found");
            return;
        }
        n += 1;
    }
}

fn part2() {
    let hasher = Md5::new_with_prefix(INPUT);

    let mut buf = itoa::Buffer::new();
    let mut n = 1;

    let mut digest = Output::<Md5>::default();

    loop {
        let mut hasher = hasher.clone();

        hasher.update(buf.format(n));
        hasher.finalize_into(&mut digest);

        let digest = digest.as_slice();

        if matches!(digest, [0, 0, 0, ..]) {
            println!("part2: {n}");
            return;
        }

        if n == i32::MAX {
            println!("part2: not found");
            return;
        }
        n += 1;
    }
}
