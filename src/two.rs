use rayon::prelude::*;

pub fn run(input: String) {
    let matchup = Match::new(input);
    let first = matchup.run(Mode::Simulate);
    println!("First: {first}");
    let second = matchup.run(Mode::Plan);
    println!("Second: {second}");
}

struct Match {
    pub game: Vec<Round>,
}

enum Mode {
    Simulate,
    Plan,
}

impl Match {
    fn new(input: String) -> Self {
        let mut buf: Vec<Round> = Vec::new();
        for i in input.lines() {
            buf.push(Round::new(i.to_string()));
        }
        Match { game: buf }
    }
    fn run(&self, mode: Mode) -> u32 {
        self.game
            .par_iter()
            .map(|r| {
                if matches!(mode, Mode::Simulate) {
                    r.resolve()
                } else {
                    r.plan()
                }
            })
            .sum()
    }
}

struct Round {
    theirs: Throw,
    ours: Throw,
    outcome: Outcome,
}

impl Round {
    fn new(r: String) -> Self {
        let theirs = if r.contains("A") {
            Throw::Rock
        } else if r.contains("B") {
            Throw::Paper
        } else {
            Throw::Scissors
        };
        let (ours, outcome) = if r.contains("X") {
            (Throw::Rock, Outcome::Lose)
        } else if r.contains("Y") {
            (Throw::Paper, Outcome::Draw)
        } else {
            (Throw::Scissors, Outcome::Win)
        };
        Round {
            theirs,
            ours,
            outcome,
        }
    }
    fn resolve(&self) -> u32 {
        let outcome = self.ours.exchange(&self.theirs);
        /*
        println!(
            "Throwing {:?} vs {:?} = {:?} \t That's {} + {}",
            self.ours,
            self.theirs,
            outcome,
            self.ours.value(),
            outcome.value()
        );
        */
        outcome.value() + self.ours.value()
    }
    fn plan(&self) -> u32 {
        let throw = self.outcome.plan_round(&self.theirs);
        /*
        println!(
            "Need {:?} vs {:?} = {:?} \t That's {} + {}",
            self.outcome,
            self.theirs,
            throw,
            throw.value(),
            self.outcome.value()
        );
        */
        throw.value() + self.outcome.value()
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl Outcome {
    fn value(&self) -> u32 {
        match &self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
    fn plan_round(&self, theirs: &Throw) -> Throw {
        match self {
            Outcome::Win => match theirs {
                Throw::Rock => Throw::Paper,
                Throw::Paper => Throw::Scissors,
                Throw::Scissors => Throw::Rock,
            },
            Outcome::Draw => match theirs {
                Throw::Rock => Throw::Rock,
                Throw::Paper => Throw::Paper,
                Throw::Scissors => Throw::Scissors,
            },
            Outcome::Lose => match theirs {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Paper,
            },
        }
    }
}

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn value(&self) -> u32 {
        match &self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn exchange(&self, rhs: &Throw) -> Outcome {
        match self {
            Throw::Rock => match rhs {
                Throw::Rock => Outcome::Draw,
                Throw::Paper => Outcome::Lose,
                Throw::Scissors => Outcome::Win,
            },
            Throw::Paper => match rhs {
                Throw::Rock => Outcome::Win,
                Throw::Paper => Outcome::Draw,
                Throw::Scissors => Outcome::Lose,
            },
            Throw::Scissors => match rhs {
                Throw::Rock => Outcome::Lose,
                Throw::Paper => Outcome::Win,
                Throw::Scissors => Outcome::Draw,
            },
        }
    }
}
