const INPUT: &str = include_str!("./input");

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;

                if floor == -1 {
                    return i + 1;
                }
            }
            _ => {}
        }
    }

    panic!("not found");
}
