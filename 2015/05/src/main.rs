#![feature(iter_array_chunks)]
#![feature(iter_map_windows)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            fn is_nice_vowels(s: &str) -> bool {
                let mut vowels = 0;

                for c in s.chars() {
                    if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                        vowels += 1;
                    }

                    if vowels == 3 {
                        return true;
                    }
                }

                false
            }

            fn is_illegal_window(window: &[char; 2]) -> bool {
                matches!(window, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'])
            }

            if !is_nice_vowels(line) {
                return false;
            }

            let mut double = false;

            let mut iter = line.chars().map_windows(|&[a, b]| [a, b]);

            for [a, b] in iter.by_ref() {
                if is_illegal_window(&[a, b]) {
                    return false;
                }

                if a == b {
                    double = true;
                    break;
                }
            }

            if !double {
                return false;
            }

            for window in iter {
                if is_illegal_window(&window) {
                    return false;
                }
            }

            true
        })
        .count()
}

fn part2(input: &str) -> usize {
    fn chunk_appears_twice(s: &str) -> bool {
        let mut map = HashMap::new();

        for (i, chunk) in s.chars().map_windows::<_, _, 2>(|w| *w).enumerate() {
            match map.entry(chunk) {
                Entry::Occupied(entry) => {
                    if i > *entry.get() + 1 {
                        return true;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }

        false
    }

    fn has_repeated_letter_xyx(s: &str) -> bool {
        s.chars().map_windows(|[a, _, b]| a == b).any(|b| b)
    }

    input.lines().filter(|line| chunk_appears_twice(line) && has_repeated_letter_xyx(line)).count()
}
