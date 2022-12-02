use aoc22::Problem;

mod solutions;

fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("no day specified")
        .parse()
        .expect("day was not an integer");

    let problem: Box<dyn Problem> = match day {
        1 => Box::new(solutions::CalorieCounting {}),
        _ => panic!("day is out of range, must be between 1 and 25"),
    };

    let input: String = std::fs::read_to_string(format!("./input/day{:0>2}.txt", day))
        .expect("unable to read input file");

    println!("day {:0>2} a: {}", day, problem.a(input.clone()));
    println!("day {:0>2} b: {}", day, problem.b(input.clone()));
}
