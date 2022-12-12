use std::fmt;
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
    println!("First: {first}")
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
    let mut eeeh: Vec<Vec<FileType>> = new_inter
        .iter()
        .rev()
        .map(|x| create_structure(x))
        .collect();
    0
}

struct ImmidiateDirectory {
    name: String,
    children: Vec<FileType>,
}

#[derive(Debug)]
enum FileType {
    Directory(String),
    NormalFile((String, u32)),
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
        if line.contains(&format!("{}", Commands::ChangeDirectory)) {
            inner_line.pop();
            if inner_line.is_empty() || inner_line.contains(&"..".to_string()) {
                break;
            } else {
                let name = inner_line.pop().expect("Nothing for name");
                pile.push(FileType::Directory(name));
            }
        } else if line.contains(&format!("{}", Commands::ListStructure)) {
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
