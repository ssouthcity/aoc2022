use std::collections::HashSet;

use problem::{print_solution, Problem};

#[derive(Debug)]
struct TuningTrouble {}

impl TuningTrouble {
    fn find_distinct_window(&self, input: Vec<char>, size: usize) -> usize {
        for (i, snap) in input.windows(size).enumerate() {
            let mut set = HashSet::new();

            if snap.iter().all(|x| set.insert(x)) {
                return size + i;
            }
        }

        unreachable!();
    }
}

impl Problem for TuningTrouble {
    fn a(&self, input: String) -> String {
        const WINDOW_SIZE: usize = 4;

        let chars: Vec<char> = input.chars().collect();

        let start_idx = self.find_distinct_window(chars, WINDOW_SIZE);

        start_idx.to_string()
    }

    fn b(&self, input: String) -> String {
        const WINDOW_SIZE: usize = 14;

        let chars: Vec<char> = input.chars().collect();

        let start_idx = self.find_distinct_window(chars, WINDOW_SIZE);

        start_idx.to_string()
    }
}

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    print_solution(TuningTrouble {}, INPUT)
}
