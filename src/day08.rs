use aoc_macros::day;

day!("Treetop Tree House", one << parse_matrix);

fn parse_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn one(trees: Vec<Vec<u32>>) -> usize {
    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];

    for y in 0..trees.len() {
        let mut highest = trees[y][0];
        visible[y][0] = true;

        // left -> right
        for x in 1..trees[y].len() {
            if trees[y][x] > highest {
                visible[y][x] = true;
                highest = trees[y][x];
            }
        }
    }

    visible
        .iter()
        .map(|row| row.iter().filter(|&v| *v).count())
        .sum()
}
