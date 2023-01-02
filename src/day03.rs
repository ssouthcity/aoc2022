use std::str::FromStr;

use aoc_macros::day;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct RuckSack(u64, u64); // we need at least 52 bits

impl FromStr for RuckSack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let half = s.len() / 2;
        let mut parts = 0u128;

        for (score, a) in ALPHABET.chars().enumerate() {
            for (i, c) in s.chars().enumerate() {
                let bit = if a == c { 1 } else { 0 };
                let shift = if i >= half { 64 } else { 0 };
                parts |= bit << (score + shift);
            }
        }

        Ok(Self((parts >> 64) as u64, parts as u64))
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
