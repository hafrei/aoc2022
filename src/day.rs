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
            "one" => Ok(Day::One),
            "two" => Ok(Day::Two),
            "three" => Ok(Day::Three),
            "four" => Ok(Day::Four),
            "five" => Ok(Day::Five),
            "six" => Ok(Day::Six),
            "seven" => Ok(Day::Seven),
            "eight" => Ok(Day::Eight),
            "nine" => Ok(Day::Nine),
            "ten" => Ok(Day::Ten),
            "eleven" => Ok(Day::Eleven),
            "twelve" => Ok(Day::Twelve),
            "thirteen" => Ok(Day::Thirteen),
            "fourteen" => Ok(Day::Fourteen),
            "fifteen" => Ok(Day::Fifteen),
            "sixteen" => Ok(Day::Sixteen),
            "seventeen" => Ok(Day::Seventeen),
            "eighteen" => Ok(Day::Eighteen),
            "nineteen" => Ok(Day::Nineteen),
            "twenty" => Ok(Day::Twenty),
            "twentyone" => Ok(Day::TwentyOne),
            "twentytwo" => Ok(Day::TwentyTwo),
            "twentythree" => Ok(Day::TwentyThree),
            "twentyfour" => Ok(Day::TwentyFour),
            "twentyfive" => Ok(Day::TwentyFive),
            _ => Err(FileLoadError::DayNotMatched),
        }
    }
}
