struct Match {
    game: Vec<Round>,
}

impl Match {
    fn new(input: String) -> Self {
        let mut buf: Vec<Round> = Vec::new();
        for i in input.lines() {
            buf.push(Round::new(i.to_string()));
        }
        Match { game: buf}
    }
    fn run(&self) -> u32 {
        let mut score = 0;
        for r in self.game.iter() {
            score += r.resolve();
        }
        score
    }
}

struct Round {
    theirs: Throw,
    ours: Throw,
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
        let ours = if r.contains("X") {
            Throw::Rock
        } else if r.contains("Y") {
            Throw::Paper
        } else {
            Throw::Scissors
        };
        Round { theirs, ours }
    }
    fn resolve(&self) -> u32 {
        let outcome = self.ours.exchange(&self.theirs);
        println!("Throwing {:?} vs {:?} = {:?} \t That's {} + {}", self.ours, self.theirs, outcome, self.ours.value(), outcome.value());
        outcome.value() + self.ours.value()
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

pub fn run(input: String) {
    let first = part_one(input.clone());
    println!("First: {first}");
}

fn part_one(input: String) -> u32 {
    let first_match = Match::new(input);
    first_match.run()
}
