use self::Day::*;
use crate::fileload::FileLoadError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Day {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

impl FromStr for Day {
    type Err = crate::fileload::FileLoadError;

    fn from_str(input: &str) -> Result<Day, Self::Err> {
        match input {
            "one" => Ok(One),
            "two" => Ok(Two),
            "three" => Ok(Three),
            "four" => Ok(Four),
            "five" => Ok(Five),
            "six" => Ok(Six),
            "seven" => Ok(Seven),
            "eight" => Ok(Eight),
            "nine" => Ok(Nine),
            "ten" => Ok(Ten),
            "eleven" => Ok(Eleven),
            "twelve" => Ok(Twelve),
            "thirteen" => Ok(Thirteen),
            "fourteen" => Ok(Fourteen),
            "fifteen" => Ok(Fifteen),
            "sixteen" => Ok(Sixteen),
            "seventeen" => Ok(Seventeen),
            "eighteen" => Ok(Eighteen),
            "nineteen" => Ok(Nineteen),
            "twenty" => Ok(Twenty),
            "twentyone" => Ok(TwentyOne),
            "twentytwo" => Ok(TwentyTwo),
            "twentythree" => Ok(TwentyThree),
            "twentyfour" => Ok(TwentyFour),
            "twentyfive" => Ok(TwentyFive),
            _ => Err(FileLoadError::DayNotMatched),
        }
    }
}
