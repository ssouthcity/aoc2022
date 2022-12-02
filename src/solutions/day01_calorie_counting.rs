use aoc22::Problem;

pub struct CalorieCounting;

impl CalorieCounting {
    fn input_to_calories(&self, input: String) -> Vec<i32> {
        return input
            .split("\n\n")
            .map(|col| {
                col.lines()
                    .map(|l| l.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .iter()
                    .sum::<i32>()
            })
            .collect();
    }
}

impl Problem for CalorieCounting {
    fn a(&self, input: String) -> String {
        let calories = self.input_to_calories(input);
        let greatest_calories = calories.iter().max().expect("input was empty");

        format!(
            "the elf that carries the most calories is carrying {} kcal",
            greatest_calories
        )
    }

    fn b(&self, input: String) -> String {
        let mut calories = self.input_to_calories(input);
        calories.sort();

        let greatest_three_calories: i32 = calories.iter().rev().take(3).sum();

        format!(
            "the three elves carrying the most calories hold {} kcal in total",
            greatest_three_calories
        )
    }
}
