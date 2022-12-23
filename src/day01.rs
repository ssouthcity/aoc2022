use aoc2022::day;

fn parse_calories(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}

fn one(calories: Vec<i32>) -> i32 {
    *calories.iter().max().unwrap()
}

fn two(mut calories: Vec<i32>) -> i32 {
    calories.sort();
    calories.iter().rev().take(3).sum()
}

day!(
    "Calorie Counting",
    one << parse_calories,
    two << parse_calories
);
