use self::FileLoadError::*;
use crate::day::Day;
use std::{env, fs::File, io::Read, str::FromStr};

const INPUTS_DIR: &str = "inputs";

pub fn read_input(day: &str, input_buffer: &mut String) -> Result<Day, FileLoadError> {
    let proj_file = format!("src/{}.rs", day);
    let mut root = env::current_dir()?;

    root.push(proj_file);

    if !root.is_file() {
        return Err(DayNotReady);
    }

    root.pop();
    root.pop();
    root.push(INPUTS_DIR);
    root.push(day);

    if root.is_file() {
        let mut f = File::open(root)?;
        f.read_to_string(input_buffer)?;
    } else {
        return Err(FileNotFound);
    }
    Day::from_str(day)
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
