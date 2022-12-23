use aoc2022::day;

use std::str::FromStr;

struct Pair(u128, u128);

impl Pair {
    fn is_covered(&self) -> bool {
        let combined = self.0 & self.1;
        combined == self.0 || combined == self.1
    }

    fn is_overlapping(&self) -> bool {
        (self.0 & self.1) != 0
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.split_once(",").unwrap();

        fn parse_range(text: &str) -> u128 {
            let (a, b) = text.split_once("-").unwrap();

            let start = a.parse::<i32>().unwrap();
            let end = b.parse::<i32>().unwrap();

            let mut result: u128 = 0;

            for i in start..=end {
                result |= 1 << i;
            }

            result
        }

        Ok(Pair(parse_range(pairs.0), parse_range(pairs.1)))
    }
}

fn parse_pair(input: &str) -> Vec<Pair> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn one(pairs: Vec<Pair>) -> i32 {
    pairs.iter().map(|p| p.is_covered() as i32).sum()
}

fn two(pairs: Vec<Pair>) -> i32 {
    pairs.iter().map(|p| p.is_overlapping() as i32).sum()
}

day!("Camp Cleanup", one << parse_pair, two << parse_pair);
