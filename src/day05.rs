use std::str::FromStr;

use aoc2022::day;

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
                    if c == ' ' {
                        continue;
                    }
                    inner.push(c);
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

fn parse_stacks(input: &str) -> (Stacks, Vec<Instruction>) {
    let (schema, instructions) = input.split_once("\n\n").unwrap();

    let stacks: Stacks = schema.parse().unwrap();

    let commands: Vec<Instruction> = instructions.lines().map(|l| l.parse().unwrap()).collect();

    (stacks, commands)
}

fn one((mut stacks, commands): (Stacks, Vec<Instruction>)) -> String {
    for c in commands {
        for _ in 0..c.quantity {
            if let Some(s) = stacks.0[c.from].pop() {
                stacks.0[c.to].push(s);
            }
        }
    }

    stacks.0.iter().map(|s| s.last().unwrap()).collect()
}

fn two((mut stacks, commands): (Stacks, Vec<Instruction>)) -> String {
    for c in commands {
        let from = &mut stacks.0[c.from];
        let mut lifted = from.split_off(from.len() - c.quantity);
        stacks.0[c.to].append(&mut lifted);
    }

    stacks.0.iter().map(|s| s.last().unwrap()).collect()
}

day!("Supply Stacks", one << parse_stacks, two << parse_stacks);
