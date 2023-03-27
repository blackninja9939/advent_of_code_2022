use color_eyre::eyre::eyre;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(eyre!("{} is not a valid character for an outcome", value)),
        }
    }
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock)
        )
    }

    fn outcome(self, other: Move) -> Outcome {
        if self.beats(other) {
            Outcome::Win
        } else if other.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(eyre!("{} is not a valid character for a move", value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl FromStr for Round {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(eyre!("expected 'theirs ours', got {s:?}"));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Round {
    fn calc_our_score(&self) -> usize {
        self.ours.score() + self.ours.outcome(self.theirs).score()
    }
}

pub fn puzzle_1(data: String) {
    let result = data
        .lines()
        .map(Round::from_str)
        .map(|round| round.unwrap().calc_our_score())
        .sum::<usize>();
    println!("Total score {}", result);
}

struct RoundGoal {
    _theirs: Move,
    target: Outcome,
    ours: Move,
}

impl FromStr for RoundGoal {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(target), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(eyre!("expected 'theirs ours', got {s:?}"));
        };

        let theirs: Move = theirs.try_into()?;
        let target: Outcome = target.try_into()?;
        let ours = target.matching_move(theirs);

        Ok(Self {
            _theirs: theirs,
            target,
            ours,
        })
    }
}

impl Move {
    fn beaten_by(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    fn draws_with(self) -> Move {
        self
    }
    fn loses_to(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl Outcome {
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.beaten_by(),
            Outcome::Draw => theirs.draws_with(),
            Outcome::Loss => theirs.loses_to(),
        }
    }
}

impl RoundGoal {
    fn calc_our_score(&self) -> usize {
        self.ours.score() + self.target.score()
    }
}

pub fn puzzle_2(data: String) {
    let result = data
        .lines()
        .map(RoundGoal::from_str)
        .map(|round| round.unwrap().calc_our_score())
        .sum::<usize>();
    println!("Total score {}", result);
}
