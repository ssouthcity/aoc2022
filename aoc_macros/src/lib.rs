#![feature(macro_metavar_expr)]

#[macro_export]
macro_rules! day_part {
    ($input:expr, $output:expr, $solver:ident, $parser:ident) => {
        let parsed_input = $parser($input);
        day_part!(parsed_input, $output, $solver);
    };
    ($input:expr, $output:expr, $solver:ident) => {
        use std::time::Instant;

        let now = Instant::now();
        let solution = $solver($input);
        let elapsed = now.elapsed();

        *$output = Some((solution, elapsed));
    };
}

#[macro_export]
macro_rules! day {
    ($name:expr, $($solver:ident$( << $parser:ident)?),*) => {
        const PART_NAMES: [&'static str; 2] = ["one", "two"];
        const CONTENT_WIDTH: usize = 40;

        pub(crate) fn execute(input: &str) {
            use aoc_macros::day_part;

            let mut results = vec![];

            $({
                let mut result = None;
                day_part!(input, &mut result, $solver$(, $parser)?);

                if let Some((solution, delta)) = result {
                    results.push((format!("{}", solution), delta));
                }
            })*

            println!("┌─{}─┐", "─".repeat(CONTENT_WIDTH));
            println!("│ {:<width$} │", $name, width = CONTENT_WIDTH);
            for (i, (solution, duration)) in results.iter().enumerate() {
                let intro = format!("{} ({}):", PART_NAMES[i], duration.as_micros());
                let remaining_width = CONTENT_WIDTH - intro.len() - 1;
                println!("│ {} {:>width$} │", intro, solution, width = remaining_width);
            }
            println!("└─{}─┘", "─".repeat(CONTENT_WIDTH));
        }
    };
}

#[macro_export]
macro_rules! generate_solution_array {
    ($($mod_name:ident),*) => {
        $(
            mod $mod_name;
        )*

        type Solution = (&'static str, for<'r> fn(&'r str));

        const SOLUTIONS: [Solution; ${count(mod_name, 0)}] = [
            $((stringify!($mod_name), $mod_name::execute), )*
        ];
    };
}
