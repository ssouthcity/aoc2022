use std::{collections::HashMap, path::PathBuf, str::FromStr};

use problem::{print_solution, Problem};

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<(String, String)>),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let cmd_line: &str = lines.nth(0).unwrap();
        let args: Vec<&str> = cmd_line.trim().split(' ').collect();

        match args[0] {
            "cd" => Ok(Self::Cd(args[1].into())),
            "ls" => {
                let processed_output: Vec<(String, String)> = lines
                    .map(|l| {
                        let (size, name) = l.split_once(' ').unwrap().into();
                        (size.into(), name.into())
                    })
                    .collect();

                Ok(Self::Ls(processed_output))
            }
            _ => unreachable!(),
        }
    }
}

struct FileData {
    name: String,
    parent: Option<Box<File>>,
}

enum File {
    Dir((FileData, Vec<File>)),
    File(FileData),
}

#[derive(Debug)]
struct NoSpaceLeftOnDevice {}

impl Problem for NoSpaceLeftOnDevice {
    fn a(&self, input: String) -> String {
        let cmds: Vec<&str> = input
            .split_inclusive("\n$")
            .map(|l| l.trim_start_matches('\n'))
            // .map(|s| s.parse().unwrap())
            .collect();

        // let mut graph: FS = FS::new();
        // let mut current_path = PathBuf::new();

        // cmds.iter().for_each(|c| match c {
        //     Command::Cd(dir) => {
        //         if dir == ".." {
        //             current_path.pop();
        //         } else {
        //             current_path.push(dir);
        //         }
        //     }
        //     Command::Ls(files) => {
        //         for file in files {
        //             let file_path = current_path.join(file.1.clone());

        //             if file.0 == "dir" {
        //                 graph.insert(file_path, File::Dir(Vec::new()));
        //             } else {
        //                 graph.entry(current_path.clone()).and_modify(|f| match f {
        //                     File::Dir(c) => c.push(file_path.clone()),
        //                     _ => unreachable!(),
        //                 });

        //                 graph.insert(file_path, File::File(file.0.parse().unwrap()));
        //             }
        //         }
        //     }
        // });

        "".to_owned()
    }

    fn b(&self, _input: String) -> String {
        "".to_owned()
    }
}

fn main() {
    print_solution(NoSpaceLeftOnDevice {}, include_str!("../input.txt"));
}
