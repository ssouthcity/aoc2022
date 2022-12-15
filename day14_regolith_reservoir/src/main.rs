#![feature(array_windows)]
use itertools::Either;
use problem::{print_solution, Problem};
use std::{collections::HashSet, str::FromStr};

fn dual_direction_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        Either::Left(start..=end)
    } else {
        Either::Right((end..=start).rev())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn range(&self, other: &Point) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        for x in dual_direction_range(self.0, other.0) {
            for y in dual_direction_range(self.1, other.1) {
                result.push(Point(x, y))
            }
        }

        result
    }

    fn down(&mut self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn bottom_left(&mut self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }

    fn bottom_right(&mut self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').unwrap();
        Ok(Self(a.parse().unwrap(), b.parse().unwrap()))
    }
}

struct Bounds {
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl Bounds {
    fn contains(&self, point: &Point) -> bool {
        (point.0 >= self.left && point.0 <= self.right)
            && (point.1 >= self.top && point.1 <= self.bottom)
    }
}

impl From<&Vec<Vec<Point>>> for Bounds {
    fn from(readings: &Vec<Vec<Point>>) -> Self {
        let mut top_left = Point(usize::MAX, usize::MAX);
        let mut bottom_right = Point(usize::MIN, usize::MIN);

        readings.iter().flatten().for_each(|p| {
            top_left.0 = top_left.0.min(p.0);
            top_left.1 = top_left.1.min(p.1);
            bottom_right.0 = bottom_right.0.max(p.0);
            bottom_right.1 = bottom_right.1.max(p.1);
        });

        Self {
            top: 0,
            right: bottom_right.0,
            bottom: bottom_right.1,
            left: top_left.0,
        }
    }
}

#[derive(Debug)]
struct RegolithReservoir {}

impl Problem for RegolithReservoir {
    fn a(&self, _input: String) -> String {
        let readings: Vec<Vec<Point>> = include_str!("../input.txt")
            .lines()
            .map(|l| l.split(" -> ").map(|r| r.parse().unwrap()).collect())
            .collect();

        let mut matrix: HashSet<Point> = HashSet::new();

        readings.iter().for_each(|set| {
            set.array_windows().for_each(|[a, b]| {
                a.range(b).iter().for_each(|p| {
                    matrix.insert(p.clone());
                });
            });
        });

        let bounds = Bounds::from(&readings);

        let pour_point = Point(500, 0);
        let mut current_point = pour_point.clone();
        let mut sand_pieces = 0;

        matrix.insert(current_point);

        'main: while bounds.contains(&current_point) {
            let new_positions = [
                current_point.down(),
                current_point.bottom_left(),
                current_point.bottom_right(),
            ];

            for pos in new_positions {
                if matrix.get(&pos).is_none() {
                    matrix.remove(&current_point);
                    current_point = pos;
                    matrix.insert(current_point);
                    continue 'main;
                }
            }

            sand_pieces += 1;
            current_point = pour_point;
        }

        sand_pieces.to_string()
    }

    fn b(&self, _input: String) -> String {
        let readings: Vec<Vec<Point>> = include_str!("../input.txt")
            .lines()
            .map(|l| l.split(" -> ").map(|r| r.parse().unwrap()).collect())
            .collect();

        let mut matrix: HashSet<Point> = HashSet::new();

        readings.iter().for_each(|set| {
            set.array_windows().for_each(|[a, b]| {
                a.range(b).iter().for_each(|p| {
                    matrix.insert(p.clone());
                });
            });
        });

        let mut bounds = Bounds::from(&readings);
        bounds.bottom += 1;
        bounds.left = 0;
        bounds.right = usize::MAX;

        let pour_point = Point(500, 0);
        let mut current_point = pour_point.clone();
        let mut sand_pieces = 0;

        matrix.insert(current_point);

        'main: loop {
            let new_positions = [
                current_point.down(),
                current_point.bottom_left(),
                current_point.bottom_right(),
            ];

            for pos in new_positions {
                if matrix.get(&pos).is_none() && bounds.contains(&pos) {
                    matrix.remove(&current_point);
                    current_point = pos;
                    matrix.insert(current_point);
                    continue 'main;
                }
            }

            sand_pieces += 1;

            if current_point == pour_point {
                break 'main;
            }

            current_point = pour_point;
        }

        sand_pieces.to_string()
    }
}

fn main() {
    print_solution(RegolithReservoir {}, include_str!("../input.txt"));
}
