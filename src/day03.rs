use std::collections::HashSet;
use std::str::FromStr;

use aoc_macros::day;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct RuckSack(u64, u64); // we need at least 52 bits

impl FromStr for RuckSack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.as_bytes();
        // let left: HashSet<char> = chars.chunks(chars.len() / 2).collect();
        // let right: HashSet<char> = chars.chunks(chars.len() / 2).collect();

        let mut a: u64 = 0;
        let mut b: u64 = 0;

        for (i, x) in ALPHABET.chars().enumerate() {
            if left.contains(&x) {
                a |= 1 << i;
            }

            if right.contains(&x) {
                b |= 1 << i;
            }
        }

        Ok(Self(a, b))
    }
}

fn priority_score(items: Vec<u64>) -> i32 {
    let mut score: i32 = 0;

    for r in items {
        for i in 0..64 {
            if (((r & (1 << i)) >> i) & 1) != 0 {
                score += i + 1;
            }
        }
    }

    score
}

fn parse_rucksacks(input: &str) -> Vec<RuckSack> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn one(rucksacks: Vec<RuckSack>) -> i32 {
    let compartment_doubles: Vec<u64> = rucksacks.iter().map(|r| r.0 & r.1).collect();

    priority_score(compartment_doubles)
}

fn two(rucksacks: Vec<RuckSack>) -> i32 {
    let groups: Vec<u64> = rucksacks
        .iter()
        .map(|r| r.0 | r.1)
        .collect::<Vec<u64>>()
        .chunks(3)
        .map(|s| s[0] & s[1] & s[2])
        .collect();

    priority_score(groups)
}

day!(
    "Rucksack Reorganization",
    one << parse_rucksacks,
    two << parse_rucksacks
);
