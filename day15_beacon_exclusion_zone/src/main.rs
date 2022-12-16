use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

trait Problem {
    fn solution(&self) -> String;
}

fn print_solution(title: &'static str, a: impl Problem, b: impl Problem) {
    println!("{}", "-".repeat(title.len()));
    println!("{}", title);
    println!("a: {:>width$}", a.solution(), width = title.len() - 3);
    println!("a: {:>width$}", b.solution(), width = title.len() - 3);
    println!("{}", "-".repeat(title.len()));
}

struct Beacon {
    x: f32,
    y: f32,
    r: f32,
}

impl FromStr for Beacon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<f32> = s
            .split_whitespace()
            .into_iter()
            .filter_map(|s| {
                s.trim_end_matches([',', ':'])
                    .split('=')
                    .nth(1)
                    .map(|n| n.parse::<f32>().unwrap())
            })
            .collect();

        let a = parts[2] - parts[0];
        let b = parts[3] - parts[1];
        let radius = (a.powf(2.0) + b.powf(2.0)).sqrt();

        Ok(Self {
            x: parts[0],
            y: parts[1],
            r: radius,
        })
    }
}

struct BeaconExclusionZoneA {
    beacons: Vec<Beacon>,
}

impl FromStr for BeaconExclusionZoneA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons: Vec<Beacon> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { beacons })
    }
}

impl Problem for BeaconExclusionZoneA {
    fn solution(&self) -> String {
        let mut intersections: Vec<(i32, i32)> = Vec::new();

        for b in self.beacons.iter() {
            let (x_1, x_2) = circle_intersect_line(b, 10.0);
            if x_1.is_nan() && x_2.is_nan() {
                continue;
            }
            intersections.push((x_2.ceil() as i32, x_1.floor() as i32));
        }

        intersections.sort_by(|a, b| b.0.cmp(&a.0));

        let mut length = 0;
        let mut current_line = intersections.pop().unwrap_or_default();

        while let Some((entry, exit)) = intersections.pop() {
            if entry > current_line.1 {
                // start new line
                length += current_line.1 - current_line.0;
                current_line = (entry, exit);
            } else {
                // overlap detected
                current_line.1 = current_line.1.max(exit);
            }
        }

        length += current_line.1 - current_line.0;

        length.to_string()
    }
}

fn abc(a: f32, b: f32, c: f32) -> (f32, f32) {
    //      -b +- sqrt(pow(b, 2) - 4ac)
    // x = ----------------------------
    //                 2a

    let x_1: f32 = ((-b) + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let x_2: f32 = ((-b) - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    (x_1, x_2)
}

fn circle_intersect_line(circle: &Beacon, line: f32) -> (f32, f32) {
    // pow((x - h), 2) + pow((y - k), 2) = pow(r, 2)
    // [pow(x, 2) - 2xh + pow(h, 2)] + pow((y - k), 2) = pow(r, 2)
    // pow(x, 2) - 2xh + pow(h, 2) + pow((y - k), 2) - pow(r, 2) = 0
    // a is always 1, b is -2h, c is pow(h, 2) + pow((y - k), 2) - pow(r, 2)

    let a = 1.0;
    let b = -2.0 * circle.x;
    let c = circle.x.powf(2.0) + (line - circle.y).powf(2.0) - circle.r.powf(2.0);

    abc(a, b, c)
}

fn main() {
    let a: BeaconExclusionZoneA = include_str!("../input.txt").parse().unwrap();
    let b: BeaconExclusionZoneA = include_str!("../input_testing.txt").parse().unwrap();

    print_solution("Beacon Exclusion Zone", a, b);
}
