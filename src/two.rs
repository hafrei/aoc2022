use self::{Outcome::*, Throw::*};
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
        let theirs = if r.contains('A') {
            Rock
        } else if r.contains('B') {
            Paper
        } else {
            Scissors
        };
        let (ours, outcome) = if r.contains('X') {
            (Rock, Lose)
        } else if r.contains('Y') {
            (Paper, Draw)
        } else {
            (Scissors, Win)
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
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
    fn plan_round(&self, theirs: &Throw) -> Throw {
        match *self {
            Win => match theirs {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Draw => match theirs {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
            Lose => match theirs {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
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
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn exchange(&self, rhs: &Throw) -> Outcome {
        match *self {
            Rock => match rhs {
                Rock => Draw,
                Paper => Lose,
                Scissors => Win,
            },
            Paper => match rhs {
                Rock => Win,
                Paper => Draw,
                Scissors => Lose,
            },
            Scissors => match rhs {
                Rock => Lose,
                Paper => Win,
                Scissors => Draw,
            },
        }
    }
}
