use std::str::FromStr;

use problem::{print_solution, Problem};

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().rev().collect();

        let mut outer: Vec<Vec<char>> = vec![];

        for i in 0..lines[0].len() {
            if let Some(' ') = lines[0].chars().nth(i) {
                continue;
            }

            let mut inner: Vec<char> = vec![];

            for j in 1..lines.len() {
                if let Some(c) = lines[j].chars().nth(i) {
                    if c != ' ' {
                        inner.push(c);
                    }
                }
            }

            outer.push(inner);
        }

        Ok(Self(outer))
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        Ok(Self {
            quantity: parts[1].parse().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
        })
    }
}

#[derive(Debug)]
pub struct SupplyStacks {}

impl Problem for SupplyStacks {
    fn a(&self, input: String) -> String {
        let (schema, instructions) = input.split_once("\n\n").unwrap();

        let mut stacks = schema.parse::<Stacks>().unwrap();

        let commands: Vec<Instruction> = instructions
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect();

        commands.iter().for_each(|c| {
            for _ in 0..c.quantity {
                if let Some(s) = stacks.0[c.from].pop() {
                    stacks.0[c.to].push(s);
                }
            }
        });

        stacks.0.iter().map(|s| s[0]).collect()
    }

    fn b(&self, _input: String) -> String {
        "".to_owned()
    }
}

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    print_solution(SupplyStacks {}, INPUT);
}
