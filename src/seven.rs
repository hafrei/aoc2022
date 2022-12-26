use rayon::prelude::*;
use std::fmt;

const FIRST_PART_MINIMUM: u32 = 100_000;

#[derive(PartialEq, Eq)]
enum Commands {
    ChangeDirectory,
    ListStructure,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Commands::ListStructure => write!(f, "ls"),
            Commands::ChangeDirectory => write!(f, "cd"),
        }
    }
}

pub fn run(input: String) {
    let first = first(input);
    println!("First: {first}");
}

fn first(input: String) -> u32 {
    let new_inter = input
        .split("$")
        .map(|x| {
            x.split('\n')
                .map(|y| y.trim().to_owned())
                .rev()
                .filter(|y| !y.is_empty())
                .collect::<Vec<String>>()
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<Vec<String>>>();
    let supposed_directory: Vec<Vec<FileType>> =
        new_inter.par_iter().rev().map(create_structure).collect();
    let dirs = get_dir_sizes(&supposed_directory);
    // println!("{dirs:?}");
    dirs.into_iter()
        .filter(|x| x.size <= FIRST_PART_MINIMUM)
        .map(|x| x.size)
        .sum()
}

#[derive(Debug)]
struct ImmidiateDirectory {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum FileType {
    Within(String),
    Directory(String),
    NormalFile((String, u32)),
}

fn get_dir_sizes(supposed_directory: &[Vec<FileType>]) -> Vec<ImmidiateDirectory> {
    let mut dirs: Vec<ImmidiateDirectory> = Vec::new();
    let mut size: u32 = 0;

    for hopefully_directory in supposed_directory.iter() {
        if hopefully_directory.is_empty() {
            continue;
        } else {
            for file_descriptor in hopefully_directory.iter() {
                match file_descriptor {
                    FileType::NormalFile((_, val)) => size += val,
                    FileType::Within(name) => {
                        let new_dir = ImmidiateDirectory {
                            name: name.to_string(),
                            size,
                        };
                        dirs.push(new_dir);
                        size = 0;
                    }
                    FileType::Directory(name) => {
                        if let Some(inner_size) =
                            dirs.iter().find(|a| a.name.eq(name)).map(|x| x.size)
                        {
                            size += inner_size;
                        }
                    }
                }
            }
        }
    }
    dirs
}

fn create_structure(input: &Vec<String>) -> Vec<FileType> {
    let mut pile = Vec::new();
    for line in input.iter() {
        let mut inner_line: Vec<String> = line
            .clone()
            .split_whitespace()
            .rev()
            .map(|x| x.to_owned())
            .collect();
        if line.contains(&format!("{}", Commands::ChangeDirectory))
            && !line.chars().any(|l| l.is_numeric())
        {
            inner_line.pop();
            if inner_line.is_empty() || inner_line.contains(&"..".to_string()) {
                break;
            } else {
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::Within(name));
                break;
            }
        } else if line.contains(&format!("{}", Commands::ListStructure)) && line.len() == 2 {
            //noop
        } else {
            if inner_line.contains(&"dir".to_string()) {
                let _ = inner_line.pop().expect("Dropping dir failed");
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::Directory(name));
            } else {
                let beeh = inner_line
                    .pop()
                    .expect("Was empty")
                    .parse::<u32>()
                    .expect("Not a number");
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::NormalFile((name, beeh)));
            }
        }
    }
    pile
}
