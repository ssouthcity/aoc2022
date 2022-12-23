use std::collections::HashSet;

use aoc2022::day;

fn find_distinct_window(input: Vec<char>, size: usize) -> usize {
    for (i, snap) in input.windows(size).enumerate() {
        let mut set = HashSet::new();

        if snap.iter().all(|x| set.insert(x)) {
            return size + i;
        }
    }

    unreachable!();
}

fn parse_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn one(chars: Vec<char>) -> usize {
    const WINDOW_SIZE: usize = 4;

    find_distinct_window(chars, WINDOW_SIZE)
}

fn two(chars: Vec<char>) -> usize {
    const WINDOW_SIZE: usize = 14;

    find_distinct_window(chars, WINDOW_SIZE)
}

day!("Tuning Trouble", one << parse_chars, two << parse_chars);
