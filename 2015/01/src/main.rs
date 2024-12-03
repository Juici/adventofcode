const INPUT: &str = include_str!("./input");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut floor = 0;

    for c in INPUT.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    println!("part1: {floor}");
}

fn part2() {
    let mut floor = 0;

    let mut pos = None;

    for (i, c) in INPUT.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;

                if floor == -1 {
                    pos = Some(i + 1);
                    break;
                }
            }
            _ => {}
        }
    }

    match pos {
        Some(pos) => println!("part2: {pos}"),
        None => println!("part2: not found"),
    }
}
