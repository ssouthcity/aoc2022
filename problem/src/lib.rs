use convert_case::{Case, Casing};
use core::fmt::Debug;

pub trait Problem {
    fn a(&self, input: String) -> String;
    fn b(&self, input: String) -> String;
}

pub fn print_solution(problem: impl Problem + Debug, input: &'static str) {
    let name = format!("{:?}", problem).to_case(Case::Title);

    let a = problem.a(input.to_string());
    let b = problem.b(input.to_string());

    println!("{}", "-".repeat(name.len()));
    println!("{}", name);
    println!("a: {:>width$}\nb: {:>width$}", a, b, width = name.len() - 3);
    println!("{}", "-".repeat(name.len()));
}
