use std::str::FromStr;

use aoc_macros::day;

struct SnafuNumber(i64);

impl FromStr for SnafuNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;

        for (i, c) in s.chars().rev().enumerate() {
            let value: i64 = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return Err("invalid snafu number character"),
            };

            sum += (5 as i64).pow(i as u32) * value;
        }

        Ok(Self(sum))
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        let mut num = self.0;
        let mut converted = vec![];

        while num > 0 {
            let digit = match num % 5 {
                4 => '-',
                3 => '=',
                2 => '2',
                1 => '1',
                0 => '0',
                _ => unreachable!(),
            };

            converted.push(digit);

            num += 2;
            num /= 5;
        }

        converted.iter().rev().collect()
    }
}

fn parse_snafu_numbers(input: &str) -> Vec<SnafuNumber> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn one(snafus: Vec<SnafuNumber>) -> String {
    let result = SnafuNumber(snafus.iter().map(|s| s.0).sum());
    result.to_string()
}

day!("Full of Hot Air", one << parse_snafu_numbers);
