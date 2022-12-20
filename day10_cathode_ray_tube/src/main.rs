use std::str;
use std::{collections::VecDeque, str::FromStr};

trait Problem {
    fn a(&self) -> String;
    fn b(&self) -> String;
}

fn print_solution(title: &'static str, problem: impl Problem) {
    println!("{}", "-".repeat(title.len()));
    println!("{}", title);
    println!("a: {:>width$}", problem.a(), width = title.len() - 3);
    println!("b: {:>width$}", problem.b(), width = title.len() - 3);
    println!("{}", "-".repeat(title.len()));
}

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

struct CathodeRayTube {
    instructions: Vec<Instruction>,
}

impl FromStr for CathodeRayTube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { instructions })
    }
}

impl Problem for CathodeRayTube {
    fn a(&self) -> String {
        let mut instructions = VecDeque::from_iter(self.instructions.iter());
        let mut cycle: i32 = 0;
        let mut score: i32 = 0;
        let mut x_reg: i32 = 1;
        let mut bank: Option<&i32> = None;

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

        score.to_string()
    }

    fn b(&self) -> String {
        let mut instructions = VecDeque::from_iter(self.instructions.iter());
        let mut pixels: String = String::with_capacity(240);
        let mut x_reg: i32 = 1;
        let mut bank: Option<&i32> = None;

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

        // pixels

        let screen_rows: Vec<&str> = pixels
            .as_bytes()
            .chunks(40)
            .map(|c| str::from_utf8(c).unwrap())
            .collect();

        format!("\n{}", screen_rows.join("\n"))
    }
}

fn main() {
    let problem: CathodeRayTube = include_str!("../input.txt").parse().unwrap();
    print_solution("Cathode-Ray Tube", problem);
}
