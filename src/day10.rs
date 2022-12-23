use std::{collections::VecDeque, str::FromStr};
use std::{fs, str};

use aoc2022::day;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => Ok(Self::Noop),
            ["addx", x] => Ok(Self::Addx(x.parse().unwrap())),
            _ => Err("string does not represent an instruction"),
        }
    }
}

fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn one(mut instructions: VecDeque<Instruction>) -> i32 {
    let mut cycle: i32 = 0;
    let mut score: i32 = 0;
    let mut x_reg: i32 = 1;
    let mut bank: Option<i32> = None;

    while bank.is_some() || !instructions.is_empty() {
        cycle += 1;

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            score += x_reg * cycle;
        }

        if let Some(x) = bank {
            x_reg += x;
            bank = None;
            continue;
        }

        if let Some(Instruction::Addx(x)) = instructions.pop_front() {
            bank = Some(x);
        }
    }

    score
}

fn two(mut instructions: VecDeque<Instruction>) -> &'static str {
    let mut pixels: String = String::with_capacity(240);
    let mut x_reg: i32 = 1;
    let mut bank: Option<i32> = None;

    for i in 0..=240 {
        if x_reg.abs_diff(i % 40) <= 1 {
            pixels.insert(i as usize, '#');
        } else {
            pixels.insert(i as usize, '.');
        }

        if let Some(x) = bank {
            x_reg += x;
            bank = None;
            continue;
        }

        if let Some(Instruction::Addx(x)) = instructions.pop_front() {
            bank = Some(x);
        }
    }

    let screen_rows: Vec<&str> = pixels
        .as_bytes()
        .chunks(40)
        .map(|c| str::from_utf8(c).unwrap())
        .collect();

    let path = "output/day10.txt";

    fs::write(path, format!("\n{}", screen_rows.join("\n"))).expect("unable to write file");

    path
}

day!(
    "Cathode-Ray Tube",
    one << parse_instructions,
    two << parse_instructions
);
