use std::fmt;

const FIRST_PART_MINIMUM: u32 = 100_000;
const SECOND_PART_MINIMUM: u32 = 30_000_000;
const DISK_SPACE: u32 = 70_000_000;

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
    let mut all_dirs = get_dirs(input);
    let first: u32 = all_dirs
        .iter()
        .cloned()
        .filter(|x| x.size <= FIRST_PART_MINIMUM)
        .map(|x| x.size)
        .sum();
    println!("First: {first}");

    all_dirs.sort();

    let root_dir = all_dirs.iter().find(|x| x.name == "/").unwrap(); //please panic if root dir is gone
    let free_space = DISK_SPACE - root_dir.size;
    println!("Free space is {free_space}");
    println!("{all_dirs:#?}");
    let mut second_candidates: Vec<u32> = all_dirs
        .iter()
        .map(|x| x.size)
        .filter(|x| x >= &SECOND_PART_MINIMUM)
        .collect();
    second_candidates.sort();
    println!("{second_candidates:?}");
    let second = second_candidates.first();
    println!("Second: {second:?}");
}

fn get_dirs(input: String) -> Vec<ImmidiateDirectory> {
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
    let mut parent = "/".to_string();
    let supposed_directory: Vec<Vec<FileType>> = new_inter
        .iter()
        .map(|x| create_structure(x, &mut parent))
        .filter(|x| !x.is_empty())
        .collect();
    get_dir_sizes(&supposed_directory)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ImmidiateDirectory {
    size: u32,
    name: String,
}

#[derive(Debug)]
enum FileType {
    Directory((String, String)),       //name, parent
    NormalFile((String, String, u32)), //name, parent, size
}

fn get_dir_sizes(supposed_directory: &[Vec<FileType>]) -> Vec<ImmidiateDirectory> {
    let mut dirs: Vec<ImmidiateDirectory> = Vec::new();
    let mut temp_parent = "/".to_string();
    let mut size: u32 = 0;

    for file_descriptor in supposed_directory.iter() {
        for info in file_descriptor {
            if let FileType::NormalFile((_, parent, val)) = info {
                temp_parent = parent.to_string();
                size += val;
            };
        }
        let dir = ImmidiateDirectory {
            size,
            name: temp_parent.clone(),
        };
        size = 0;
        dirs.push(dir);
    }

    dirs.sort();

    for file_descriptor in supposed_directory.iter() {
        for info in file_descriptor {
            if let FileType::Directory((name, parent)) = info {
                let new_size = if let Some(current) = dirs.iter().find(|x| &x.name == name) {
                    current.size
                } else {
                    panic!("Oh no what how")
                };
                if let Some(parent) = dirs.iter_mut().find(|x| &x.name == parent) {
                    parent.size += new_size;
                };
            }
        }
    }
    dirs
}

fn create_structure(input: &Vec<String>, parent: &mut String) -> Vec<FileType> {
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
                *parent = inner_line.pop().expect("Nothing for name");
            }
        } else if line.contains(&format!("{}", Commands::ListStructure)) && line.len() == 2 {
            continue;
        } else {
            if inner_line.contains(&"dir".to_string())
                && !inner_line.iter().any(|w| w.chars().any(|c| c.is_numeric()))
            {
                let _ = inner_line.pop().expect("Dropping dir failed");
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::Directory((name, parent.clone())));
            } else {
                let size = inner_line
                    .pop()
                    .expect("Was empty")
                    .parse::<u32>()
                    .expect("Not a number");
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::NormalFile((name, parent.clone(), size)));
            }
        }
    }
    pile
}
