use std::borrow::Cow;
use std::fmt;

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

#[derive(Debug, Clone)]
struct FileDetails {
    name: Cow<'static, str>,
    parent: Cow<'static, str>,
    size: u32,
    depth: i32,
    file_type: FileType,
}

impl FileDetails {
    fn new(name: String, parent: String, size: u32, depth: i32, file_type: FileType) -> Self {
        Self {
            name: name.into(),
            parent: parent.into(),
            size,
            depth,
            file_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FileType {
    Directory,
    NormalFile,
}

pub fn run(input: String) {
    let mut max_depth: i32 = 0;
    let mut all_dirs = get_dirs(input, &mut max_depth);
    let first: u32 = all_dirs
        .iter()
        .cloned()
        .filter(|x| x.file_type == FileType::Directory && x.size <= FIRST_PART_MINIMUM)
        .map(|x| x.size)
        .sum();
    println!("First: {first}");

    let mut root_size = 0;
    if let Some(root) = all_dirs.iter().find(|x| x.name == "/") {
        root_size = root.size;
    };

    let space_remaining = DISK_SPACE - root_size;
    println!("Root size is {root_size}, with remaining: {space_remaining}");
    let needed = SECOND_PART_MINIMUM - space_remaining;
    // let needed = if space_remaining > SECOND_PART_MINIMUM {
    //     space_remaining - SECOND_PART_MINIMUM
    // } else {
    //     SECOND_PART_MINIMUM - space_remaining
    // };
    println!("Needed: {needed}");
    let second: u32 = all_dirs.clone()
        .iter_mut()
        .filter(|x| x.file_type == FileType::Directory)
        .filter(|x| x.size >= needed)
        .map(|x| x.size)
        .min()
        .unwrap_or(0);
    println!("Second: {second}"); //2786160 is too high
                                  //also womp womp 29011935
    let bup: Vec<FileDetails> = all_dirs
        .into_iter()
        .filter(|x| x.file_type == FileType::Directory /*&& x.size >= needed*/).collect();
    println!("{bup:#?}");
}

fn get_dirs(input: String, max_depth: &mut i32) -> Vec<FileDetails> {
    let new_inter = input
        .split('$')
        .map(|x| {
            x.split('\n')
                .map(|y| y.trim().to_owned())
                .filter(|y| !y.is_empty())
                .collect::<Vec<String>>()
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<Vec<String>>>();
    let base_dir = "".to_string();
    let mut parent = Vec::new();
    parent.push(base_dir);
    let mut depth: i32 = 0;
    let mut supposed_directory: Vec<FileDetails> = new_inter
        .iter()
        .map(|x| create_structure(x, &mut parent, &mut depth, max_depth))
        .filter(|x| !x.is_empty())
        .flatten()
        .collect();
    get_dir_sizes(&mut supposed_directory, *max_depth, FileType::NormalFile);
    get_dir_sizes(&mut supposed_directory, *max_depth, FileType::Directory);
    supposed_directory
}

fn create_structure(
    input: &Vec<String>,
    parent: &mut Vec<String>,
    depth: &mut i32,
    max_depth: &mut i32,
) -> Vec<FileDetails> {
    let mut pile: Vec<FileDetails> = Vec::new();
    for (e, line) in input.iter().enumerate() {
        println!("Line {e} is {line:?}");
        let mut inner_line: Vec<String> = line
            .clone()
            .split_whitespace()
            .rev()
            .map(|x| x.to_owned())
            .collect();

        if line.contains(&format!("{}", Commands::ParentDirectory)) {
            println!("   so reduce depth by one");
            *depth -= 1;
            println!("        and parent was {:?}", parent.last());
            parent.pop();
            println!("        and update parent to {:?}", parent.last());
        } else if line.contains(&format!("{}", Commands::ChangeDirectory)) {
            println!("   so we're changing directory");
            if line.contains('/') {
                println!("        and adding root");
                pile.push(FileDetails {
                    name: "/".into(),
                    parent: "/.".into(),
                    size: 0,
                    depth: 0,
                    file_type: FileType::Directory,
                });
            }

            println!("   going to increase depth");
            *depth += 1;
            if *depth >= *max_depth {
                println!("        and update max depth to {}", *depth);
                *max_depth = *depth;
            }

            let _ = inner_line.pop(); //Drop the "cd"
            parent.push(inner_line.pop().expect("Nothing for name"));
            println!("        and update parent to {:?}", parent.last());
        } else if line.contains(&format!("{}", Commands::ListStructure)) && line.len() == 2 {
            println!("   so treat this as a noop");
            continue;
        } else if inner_line.contains(&"dir".to_string())
            && !inner_line.iter().any(|w| w.chars().any(|c| c.is_numeric()))
        {
            println!("   so we're adding a directory");
            let _ = inner_line.pop().expect("Dropping dir failed");
            let name = inner_line.pop().expect("Nothing for name");
            println!("        and the name is {name}");
            if let Some(rent) = parent.last() {
                pile.push(FileDetails::new(
                    name,
                    rent.to_owned(),
                    0,
                    *depth,
                    FileType::Directory,
                ));
            };
        } else {
            println!("   so we're adding a file");
            let size = inner_line
                .pop()
                .expect("Was empty")
                .parse::<u32>()
                .expect("Not a number");
            let name = inner_line.pop().expect("Nothing for name");
            println!("        and the name is {name}");
            if let Some(rent) = parent.last() {
                pile.push(FileDetails::new(
                    name,
                    rent.to_owned(),
                    size,
                    *depth,
                    FileType::NormalFile,
                ));
            };
        }
    }
    pile
}

fn get_dir_sizes(
    directory: &mut Vec<FileDetails>,
    max_depth: i32,
    filter: FileType) {

    for i in (0..=max_depth).rev() {
    let dir_cache: Vec<FileDetails> = directory.iter().cloned().collect();
        println!("{i}");
        for info in dir_cache
            .iter()
            .filter(|x| x.depth == i && x.file_type == filter)
        {
            println!("Have {info:?}");
            if let Some(parent) = directory.iter_mut().find(|x| x.name == info.parent) {
                println!("     Found {parent:?}");
                parent.size += info.size;
                println!("     Now {parent:?}\n");
            } else {

                    panic!("Didn't find a parent! That's not possible unles it's root");
            }
        }
    }
}
