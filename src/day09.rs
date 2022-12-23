use std::{collections::HashSet, str::FromStr};

use aoc2022::day;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err("string can not be converted to Direction"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn diff(&self, other: &Point) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }

    fn distant(&self, other: &Point) -> bool {
        !(self == other) && !self.adjacent(other) && !self.diagonal(other)
    }

    fn adjacent(&self, other: &Point) -> bool {
        ((self.0.abs_diff(other.0) == 1) && (self.1.abs_diff(other.1) == 0))
            || ((self.1.abs_diff(other.1) == 1) && (self.0.abs_diff(other.0) == 0))
    }

    fn diagonal(&self, other: &Point) -> bool {
        (self.0.abs_diff(other.0) == 1) && (self.1.abs_diff(other.1) == 1)
    }

    fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Self {
            knots: vec![Point(0, 0); size],
        }
    }

    fn step(&mut self, dir: Direction) {
        self.knots[0] = self.knots[0].step(dir);

        for i in 1..self.knots.len() {
            let knot = self.knots[i];
            let front = self.knots[i - 1];

            if !knot.distant(&front) {
                break;
            }

            let diff = front.diff(&knot);
            let new_x = knot.0 + diff.0.clamp(-1, 1);
            let new_y = knot.1 + diff.1.clamp(-1, 1);

            self.knots[i] = Point(new_x, new_y);
        }
    }
}

fn parse_routine(input: &str) -> Vec<(Direction, u8)> {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn one(routine: Vec<(Direction, u8)>) -> usize {
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert(tail);

    for (d, n) in routine {
        for _ in 0..n {
            let new_head = head.step(d);

            if new_head.distant(&tail) {
                tail = head;
                visited.insert(tail);
            }

            head = new_head;
        }
    }

    visited.len()
}

fn two(routine: Vec<(Direction, u8)>) -> usize {
    let mut rope = Rope::new(10);
    let mut visited: HashSet<Point> = HashSet::new();

    for (d, n) in routine.iter() {
        for _ in 0..n.clone() {
            rope.step(d.clone());

            if let Some(tail) = rope.knots.last() {
                visited.insert(tail.clone());
            }
        }
    }

    visited.len()
}

day!("Rope Bridge", one << parse_routine, two << parse_routine);
