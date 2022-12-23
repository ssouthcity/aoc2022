#[macro_export]
macro_rules! day_part {
    ($input:expr, $output:expr, $solver:ident, $parser:ident) => {
        let parsed_input = $parser($input);
        day_part!(parsed_input, $output, $solver);
    };
    ($input:expr, $output:expr, $solver:ident) => {
        *$output = Some($solver($input));
    };
}

#[macro_export]
macro_rules! day {
    ($name:expr, $($solver:ident$( << $parser:ident)?),*) => {
        const PART_NAMES: [&'static str; 2] = ["one", "two"];
        const CONTENT_WIDTH: usize = 30;

        pub(crate) fn execute(input: &str) {
            use aoc2022::day_part;


            println!("┌─{}─┐", "─".repeat(CONTENT_WIDTH));
            println!("│ {:<width$} │", $name, width = CONTENT_WIDTH);

            let mut names = PART_NAMES.iter();

            $({
                let mut result = None;
                day_part!(input, &mut result, $solver$(, $parser)?);
                let part_name = names.next().unwrap();
                println!("│ {}: {:>width$} │", part_name, result.unwrap(), width = CONTENT_WIDTH - part_name.len() - 2);
            })*

            println!("└─{}─┘", "─".repeat(CONTENT_WIDTH));
        }
    };
}

#[macro_export]
macro_rules! generate_main {
    ($($mod_name:ident),*) => {
        $(
            mod $mod_name;
        )*

        fn main() {
            use std::fs;

            $(
                let path = format!("./input/{}.txt", stringify!($mod_name));
                let input = fs::read_to_string(path).unwrap();
                $mod_name::execute(input.as_str());
            )*
        }
    };
}
