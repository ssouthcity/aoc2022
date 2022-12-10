use std::str::FromStr;

use aoc22::Problem;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct RuckSack(u64, u64); // we need at least 52 bits

impl RuckSack {
    fn in_both_pockets(&self) -> u64 {
        self.0 & self.1
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

impl Problem for RucksackReorganization {
    fn a(&self, input: String) -> String {
        let rucksacks: Vec<RuckSack> = input
            .lines()
            .map(|l| l.parse::<RuckSack>().unwrap())
            .collect();

        let mut score: i32 = 0;

        for r in rucksacks {
            let both = r.in_both_pockets();
            for i in 0..64 {
                if (((both & (1 << i)) >> i) & 1) != 0 {
                    score += i + 1;
                }
            }
        }

        score.to_string()
    }

    fn b(&self, _input: String) -> String {
        "".to_owned()
    }
}
