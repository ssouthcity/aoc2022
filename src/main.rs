#![feature(macro_metavar_expr)]
#![feature(array_windows)]
use std::{env, fs};

use aoc_macros::generate_solution_array;

generate_solution_array!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
    // day14,
    day25
);

fn main() {
    let args: Vec<usize> = env::args().skip(1).map(|a| a.parse().unwrap()).collect();

    SOLUTIONS
        .iter()
        .enumerate()
        .filter(|(i, _)| args.is_empty() || args.contains(&(i + 1)))
        .for_each(|(_, (name, exec))| {
            let path = format!("./input/{}.txt", name);
            let input = fs::read_to_string(path).unwrap();
            exec(input.as_str());
        });
}
