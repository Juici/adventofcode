use std::sync::LazyLock;

use regex::Regex;

const INPUT: &str = include_str!("./input");

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let mut sum = 0;

    for capture in REGEX.captures_iter(input) {
        let (_, [left, right]) = capture.extract();

        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();

        sum += left * right;
    }

    sum
}

fn part2(input: &str) -> u64 {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap());

    let mut sum = 0;
    let mut enabled = true;

    for capture in REGEX.captures_iter(input) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let left = &capture[1];
                let right = &capture[2];

                let left = left.parse::<u64>().unwrap();
                let right = right.parse::<u64>().unwrap();

                sum += left * right;
            }
            _ => {}
        }
    }

    sum
}
