use aoc_macros::day;

fn parse_rps(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();

            let opponent = "ABC".find(a).unwrap();
            let player = "XYZ".find(b).unwrap();

            (opponent, player)
        })
        .collect()
}

fn one(matches: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for (opponent, player) in matches {
        let mut results = [3, 6, 0];
        results.rotate_right(opponent);

        let result = results[player];

        sum += (player + 1) + result;
    }

    sum
}

fn two(matches: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    let scores = [0, 3, 6];

    for (opponent, result) in matches {
        let mut results = [3, 1, 2];
        results.rotate_left(opponent);

        let player = results[result];

        sum += scores[result] + player
    }

    sum
}

day!("Rock Paper Scissors", one << parse_rps, two << parse_rps);
