use std::fmt;
use std::borrow::Cow;

const FIRST_PART_MINIMUM: u32 = 100_000;
const SECOND_PART_MINIMUM: u32 = 30_000_000;
const DISK_SPACE: u32 = 70_000_000;

#[derive(PartialEq, Eq)]
enum Commands {
    ChangeDirectory,
    ListStructure,
    ParentDirectory,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Commands::ListStructure => write!(f, "ls"),
            Commands::ChangeDirectory => write!(f, "cd"),
            Commands::ParentDirectory => write!(f, "cd .."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ImmidiateDirectory {
    size: u32,
    depth: i32,
    name: String,
}

#[derive(Debug, Clone)]
struct FileDetails {
    name: Cow<'static, str>,
    parent: Cow<'static, str>,
    size: u32,
    depth: i32,
}

impl FileDetails {
    fn new(name: String, parent: String, size: u32, depth: i32) -> Self {
        Self {
            name: name.into(),
            parent: parent.into(),
            size,
            depth,
        }
    }
}

#[derive(Debug, Clone)]
enum FileType {
    Directory(FileDetails),
    NormalFile(FileDetails),
}

pub fn run(input: String) {
    let mut max_depth: i32 = 0;
    let mut all_dirs = get_dirs(input, &mut max_depth);
    let first: u32 = all_dirs
        .iter()
        .cloned()
        .filter(|x| x.size <= FIRST_PART_MINIMUM)
        .map(|x| x.size)
        .sum();
    println!("First: {first}");
    // println!("Max depth: {max_depth}");
    // println!("\n\n\n\n\n");

    all_dirs.sort();

    // let root_dir = all_dirs.iter().find(|x| x.name == *"/").unwrap(); //please panic if root dir is gone
    // let free_space = DISK_SPACE - root_dir.size;
    // // println!("{root_dir:?}");
    // println!("Free space is {free_space}");
    // let mut second_candidates: Vec<u32> = all_dirs
    //     .iter()
    //     .map(|x| x.size)
    //     .filter(|x| x + free_space >= SECOND_PART_MINIMUM)
    //     .collect();
    // second_candidates.sort();
    // println!("{second_candidates:?}");
    // let second = second_candidates.first();
    // println!("Second: {second:?}");
}

fn get_dirs(input: String, max_depth: &mut i32) -> Vec<ImmidiateDirectory> {
    let new_inter = input
        .split('$')
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
    let mut depth: i32 = 0;
    let supposed_directory: Vec<FileType> = new_inter
        .iter()
        .map(|x| create_structure(x, &mut parent, &mut depth, max_depth))
        .filter(|x| !x.is_empty())
        .flatten()
        .collect();
    get_dir_sizes(&supposed_directory, *max_depth)
}

fn get_dir_sizes(supposed_directory: &Vec<FileType>, max_depth: i32) -> Vec<ImmidiateDirectory> {
    //By this point, supposed_directory is an accurate representation of the file tree.
    let mut dirs: Vec<ImmidiateDirectory> = Vec::new();
    let mut size: u32 = 0;
    let mut temp_parent = ".".to_string();
    let mut temp_depth = 0;

    let new_dirs: Vec<FileType> = supposed_directory.iter().cloned().filter(|x| matches!(x, FileType::NormalFile(_))).collect();

    for i in (0..=max_depth).rev() {
        for info in supposed_directory.iter() {
            if let FileType::NormalFile(file_details) = info {
                if file_details.depth != i+1 {
                    // println!("Skipping {info:?},\ndepth should be {}", i+1);
                    continue;
                }
                // println!("{info:?}");
                temp_parent = file_details.parent.to_string();
                size += file_details.size;
                temp_depth = file_details.depth;
            };
        }
        let dir = ImmidiateDirectory {
            size,
            depth: temp_depth - 1,
            name: temp_parent.clone(),
        };

        dirs.push(dir);
    }

    dirs.sort();
    //by this point, the dirs are wrong. a has vanished
    // and the sizes are all wrong
    println!("{dirs:#?}\n");

    for i in (0..=max_depth).rev() {
        println!("{i}");
        for info in supposed_directory.iter() {
            if let FileType::Directory(file_detail) = info {
                if file_detail.depth != i {
                    continue;
                }
                println!("{info:?}");
                let new_size =
                    if let Some(current) = dirs.iter().find(|x| x.name == file_detail.name) {
                        println!("Current: {current:?}");
                        current.size
                    } else {
                        0
                    };
                println!("Looking for {}", file_detail.parent);
                if let Some(parent) = dirs
                    .iter_mut()
                    .find(|x| (x.name == file_detail.parent))
                {
                    println!("Parent: {parent:?}");
                    parent.size += new_size;
                    println!("Parent: {parent:?\n}");
                };
            }
        }
    }
    dirs
}

fn create_structure(
    input: &Vec<String>,
    parent: &mut String,
    depth: &mut i32,
    max_depth: &mut i32,
) -> Vec<FileType> {
    let mut pile = Vec::new();
    for line in input.iter() {
        let mut inner_line: Vec<String> = line
            .clone()
            .split_whitespace()
            .rev()
            .map(|x| x.to_owned())
            .collect();

        if line.contains(&format!("{}", Commands::ParentDirectory)) {
            *depth -= 1;
        } else if line.contains(&format!("{}", Commands::ChangeDirectory)) {
            if line.contains('/')  {
                pile.push(FileType::Directory(FileDetails {
                    name: "/".into(),
                    parent: "/.".into(),
                    size: 0,
                    depth: 0,
                }));
            }

            *depth += 1;
            if *depth >= *max_depth {
                *max_depth = *depth;
            }

            let _ = inner_line.pop(); //Drop the "cd"
            *parent = inner_line.pop().expect("Nothing for name");
        } else if line.contains(&format!("{}", Commands::ListStructure)) && line.len() == 2 {
            continue;
        } else if inner_line.contains(&"dir".to_string())
            && !inner_line.iter().any(|w| w.chars().any(|c| c.is_numeric()))
        //Some files have "dir" in the name, but no dir name contains numbers
        {
            let _ = inner_line.pop().expect("Dropping dir failed");
            let name = inner_line.pop().expect("Nothing for name");
            pile.push(FileType::Directory(FileDetails::new(
                name,
                parent.clone(),
                0,
                *depth,
            )));
        } else {
            let size = inner_line
                .pop()
                .expect("Was empty")
                .parse::<u32>()
                .expect("Not a number");
            let name = inner_line.pop().expect("Nothing for name");
            pile.push(FileType::NormalFile(FileDetails::new(
                name,
                parent.clone(),
                size,
                *depth,
            )));
        }
    }
    pile
}
