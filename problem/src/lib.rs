pub trait Problem {
    fn a(&self, input: String) -> String;
    fn b(&self, input: String) -> String;
}

pub fn print_solution(problem: impl Problem, input: &'static str) {
    let a = problem.a(input.to_string());
    let b = problem.b(input.to_string());
    let max_len = a.len().max(b.len());

    println!("a: {:>width$}\nb: {:>width$}", a, b, width = max_len);
}
