#![warn(clippy::pedantic)]
use aoc2022::{day::Day::*, eight, fileload, five, four, one, seven, six, three, two};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let mut input_buffer = String::new();
    let day = match env::args().nth(1) {
        None => {
            eprintln!("Can't do something with nothing! Give me a day to run!");
            return ExitCode::FAILURE;
        }
        Some(input) => match fileload::read_input(&input.to_lowercase(), &mut input_buffer) {
            Ok(day_enum) => day_enum,
            Err(e) => {
                eprintln!("Error occurred: {e:?}");
                return ExitCode::FAILURE;
            }
        },
    };

    match day {
        One => one::run(input_buffer),
        Two => two::run(input_buffer),
        Three => three::run(input_buffer),
        Four => four::run(input_buffer),
        Five => five::run(input_buffer),
        Six => six::run(input_buffer),
        Seven => seven::run(input_buffer),
        Eight => eight::run(input_buffer),
        _ => unreachable!(),
    }
    ExitCode::SUCCESS
}
