use std::str::FromStr;

use problem::{print_solution, Problem};

const INPUT: &'static str = include_str!("../input.txt");

#[derive(Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn index(self) -> usize {
        self as usize
    }

    fn points(self) -> usize {
        self.index() + 1
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("'{}' is not a valid shape", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum MatchResult {
    Draw,
    Victory,
    Defeat,
}

impl MatchResult {
    fn index(self) -> usize {
        self as usize
    }

    fn points(self) -> usize {
        match self {
            Self::Defeat => 0,
            Self::Draw => 3,
            Self::Victory => 6,
        }
    }
}

impl FromStr for MatchResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Defeat),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Victory),
            _ => Err(format!("'{}' is not a valid match result", s)),
        }
    }
}

// trait IndexPointable {
//     fn index(&self) -> usize;
//     fn points(&self) -> usize;
// }

// struct MatchNew {
//     player: Box<dyn IndexPointable>,
//     opponent: Box<dyn IndexPointable>,
// }

// impl MatchNew {
//     fn score(&self, mut result_lookup: [Box<dyn IndexPointable>; 3]) -> usize {
//         result_lookup.rotate_right(self.opponent.index());

//         self.player.points() + result_lookup[self.player.index()].points()
//     }
// }

struct Match(Shape, Shape);

impl Match {
    fn score(&self) -> usize {
        let mut results = [MatchResult::Draw, MatchResult::Victory, MatchResult::Defeat];
        results.rotate_right(self.0.index());

        let result = results[self.1.index()];

        self.1.points() + result.points()
    }
}

impl FromStr for Match {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").expect("unable to parse match");

        Ok(Self(Shape::from_str(a)?, Shape::from_str(b)?))
    }
}

struct PartialMatch(Shape, MatchResult);

impl PartialMatch {
    fn score(&self) -> usize {
        let mut results = [Shape::Rock, Shape::Paper, Shape::Scissors];
        results.rotate_left(self.0.index());

        let result = results[self.1.index()];

        result.points() + self.1.points()
    }
}

impl FromStr for PartialMatch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").expect("unable to parse partial match");

        Ok(Self(Shape::from_str(a)?, MatchResult::from_str(b)?))
    }
}

#[derive(Debug)]
pub struct RockPaperScissors;

impl Problem for RockPaperScissors {
    fn a(&self, input: String) -> String {
        let matches: Vec<Match> = input.lines().map(|l| l.parse().unwrap()).collect();

        let score: usize = matches.iter().map(|m| m.score()).sum();

        score.to_string()
    }

    fn b(&self, input: String) -> String {
        let matches: Vec<PartialMatch> = input.lines().map(|l| l.parse().unwrap()).collect();

        let score: usize = matches.iter().map(|m| m.score()).sum();

        score.to_string()
    }
}

fn main() {
    print_solution(RockPaperScissors {}, INPUT);
}
