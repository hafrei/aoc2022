use crate::day::Day;
use std::{
    env,
    fs::{read_dir, DirEntry, File},
    io::{self, Read},
    path::Path,
    str::FromStr,
};

const INPUTS_DIR: &str = "inputs";

pub fn read_input(day: &str, input_buffer: &mut String) -> Result<Day, FileLoadError> {
    let proj_file = format!("src/{}.rs", day);
    //let proj_file_path = Path::new(&proj_file);
    let mut root = env::current_dir()?;

    //visit_dirs(proj_file_path);

    root.push(proj_file);

    if !root.is_file() {
        return Err(FileLoadError::DayNotReady);
    }

    root.pop();
    root.pop();
    root.push(INPUTS_DIR);
    root.push(&day);

    if root.is_file() {
        let mut f = File::open(root)?;
        f.read_to_string(input_buffer)?;
    } else {
        return Err(FileLoadError::FileNotFound);
    }

    Ok(Day::from_str(day)?)
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum FileLoadError {
    IOError(std::io::Error),
    FileNotFound,
    DayNotReady,
    DayNotMatched,
}

impl From<std::io::Error> for FileLoadError {
    fn from(e: std::io::Error) -> FileLoadError {
        FileLoadError::IOError(e)
    }
}
