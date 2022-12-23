use std::str::FromStr;

use aoc2022::day;

#[derive(Copy, Clone)]
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

#[derive(Clone, Copy)]
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

fn parse_matches(input: &str) -> Vec<Match> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn parse_partial_matches(input: &str) -> Vec<PartialMatch> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn one(matches: Vec<Match>) -> usize {
    matches.iter().map(|m| m.score()).sum()
}

fn two(matches: Vec<PartialMatch>) -> usize {
    matches.iter().map(|m| m.score()).sum()
}

day!(
    "Rock Paper Scissors",
    one << parse_matches,
    two << parse_partial_matches
);
