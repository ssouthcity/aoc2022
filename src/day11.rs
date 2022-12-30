use std::{collections::VecDeque, str::FromStr};

use aoc_macros::day;

type Item = i32;

enum Operation {
    Add(Item),
    Multiply(Item),
    Square,
}

impl Operation {
    fn execute(&self, old: Item) -> Item {
        match self {
            Self::Add(x) => old + x,
            Self::Multiply(x) => old * x,
            Self::Square => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split(' ').collect::<Vec<&str>>()[2..] {
            ["old", "*", "old"] => Ok(Self::Square),
            ["old", "*", x] => Ok(Self::Multiply(x.parse().unwrap())),
            ["old", "+", x] => Ok(Self::Add(x.parse().unwrap())),
            _ => Err("invalid operation"),
        }
    }
}

struct Test(i32, usize, usize);

impl Test {
    fn run(&self, item: Item) -> usize {
        if item % self.0 == 0 {
            self.1
        } else {
            self.2
        }
    }
}

impl FromStr for Test {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i32> = s
            .lines()
            .map(|l| l.split(' ').last().unwrap().parse().unwrap())
            .collect();

        Ok(Self(nums[0], nums[1] as usize, nums[2] as usize))
    }
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    business: i32,
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // skip first line

        let (_, raw_items) = lines.next().unwrap().split_once(':').unwrap();
        let items: VecDeque<Item> = raw_items
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let (_, raw_operation) = lines.next().unwrap().split_once(':').unwrap();
        let operation: Operation = raw_operation.parse().unwrap();

        let raw_test: Vec<&str> = lines.take(3).collect();
        let test: Test = raw_test.join("\n").parse().unwrap();

        Ok(Self {
            items,
            operation,
            test,
            business: 0,
        })
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|s| s.parse().unwrap()).collect()
}

fn one(mut monkeys: Vec<Monkey>) -> i32 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut new_item = monkeys[i].operation.execute(item);

                new_item /= 3;

                let throw_to = monkeys[i].test.run(new_item);

                monkeys[throw_to].items.push_back(new_item);

                monkeys[i].business += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.business.cmp(&a.business));

    monkeys[0].business * monkeys[1].business
}

day!("Monkey in the Middle", one << parse_monkeys);
