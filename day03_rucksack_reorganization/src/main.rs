use std::str::FromStr;

use problem::Problem;

const INPUT: &'static str = include_str!("../input.txt");
const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct RuckSack(u64, u64); // we need at least 52 bits

impl RuckSack {
    fn in_both_pockets(&self) -> u64 {
        self.0 & self.1
    }

    fn combine_pockets(&self) -> u64 {
        self.0 | self.1
    }
}

impl FromStr for RuckSack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left: Vec<char> = s.chars().collect();
        let right: Vec<char> = left.split_off(left.len() / 2);

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

pub struct RucksackReorganization {}

impl RucksackReorganization {
    fn priority_score(&self, items: Vec<u64>) -> i32 {
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
}

impl Problem for RucksackReorganization {
    fn a(&self, input: String) -> String {
        let rucksacks: Vec<RuckSack> = input.lines().map(|l| l.parse().unwrap()).collect();

        let compartment_doubles: Vec<u64> = rucksacks.iter().map(|r| r.in_both_pockets()).collect();

        let score = self.priority_score(compartment_doubles);

        score.to_string()
    }

    fn b(&self, input: String) -> String {
        let rucksacks: Vec<RuckSack> = input.lines().map(|l| l.parse().unwrap()).collect();

        let groups: Vec<u64> = rucksacks
            .iter()
            .map(|r| r.combine_pockets())
            .collect::<Vec<u64>>()
            .chunks(3)
            .map(|s| s[0] & s[1] & s[2])
            .collect();

        let score = self.priority_score(groups);

        score.to_string()
    }
}

fn main() {
    problem::print_solution(RucksackReorganization {}, INPUT);
}
