#![warn(clippy::pedantic)]
use aoc2022::{day::Day, fileload, one, three, two};
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
        Day::One => one::run(input_buffer),
        Day::Two => two::run(input_buffer),
        Day::Three => three::run(input_buffer),
        _ => {} //Noop, many checks will have made it so
    }
    ExitCode::SUCCESS
}
