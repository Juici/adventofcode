use regex::Regex;

const INPUT: &str = include_str!("./input");

fn main() {
    part1();
    part2();
}

fn part1() {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for capture in regex.captures_iter(INPUT) {
        let (_, [left, right]) = capture.extract();

        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();

        sum += left * right;
    }

    println!("part 1: {sum}");
}

fn part2() {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for capture in regex.captures_iter(INPUT) {
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

    println!("part 1: {sum}");
}
