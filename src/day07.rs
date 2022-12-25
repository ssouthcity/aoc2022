use std::collections::HashMap;

use aoc2022::day;

day!("No Space Left on Device", one << parse_terminal_log);

enum TerminalLine<'a> {
    Cd(&'a str),
    Ls,
    Output(&'a str, &'a str),
}

fn parse_terminal_log(input: &str) -> Vec<TerminalLine> {
    input
        .lines()
        .map(|l| match l.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", a] => TerminalLine::Cd(a),
            ["$", "ls"] => TerminalLine::Ls,
            [size, name] => TerminalLine::Output(size, name),
            _ => unreachable!(),
        })
        .collect()
}

#[derive(PartialEq)]
enum FileType {
    Dir,
    File(u64),
}

type FileSystem<'a> = HashMap<&'a str, (FileType, Vec<&'a str>)>;

fn build_fs_graph(lines: Vec<TerminalLine>) -> FileSystem {
    let mut fs = FileSystem::new();
    let mut current_dir = Vec::new();

    for line in lines {
        match line {
            TerminalLine::Cd(dir) => {
                if dir == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(dir);
                }
            }
            TerminalLine::Output(size, name) => {
                fs.entry(current_dir.last().unwrap())
                    .or_insert((FileType::Dir, vec![]))
                    .1
                    .push(name);

                if let Ok(size) = size.parse::<u64>() {
                    fs.insert(name, (FileType::File(size), vec![]));
                } else {
                    fs.insert(name, (FileType::Dir, vec![]));
                }
            }
            TerminalLine::Ls => {}
        }
    }

    fs
}

fn file_size(fs: &FileSystem, key: &str) -> u64 {
    match fs.get(key).unwrap() {
        (FileType::Dir, children) => children.iter().map(|c| file_size(fs, c)).sum(),
        (FileType::File(size), _) => *size,
    }
}

fn one(lines: Vec<TerminalLine>) -> u64 {
    let fs = build_fs_graph(lines);
    let mut dirs = vec![];

    for key in fs.keys() {
        if let Some((FileType::Dir, _)) = fs.get(key) {
            let size = file_size(&fs, key);

            if size <= 100000 {
                dirs.push(size);
            }
        }
    }

    dirs.iter().sum()
}
