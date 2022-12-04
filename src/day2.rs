use crate::day2::MatchResult::*;
use crate::day2::Shape::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ParseShapeError;

impl Display for ParseShapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error oop")
    }
}

impl Error for ParseShapeError {}

impl FromStr for Shape {
    type Err = ParseShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(ParseShapeError),
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for MatchResult {
    type Err = ParseShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(ParseShapeError),
        }
    }
}

impl MatchResult {
    fn score(self) -> usize {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }

    fn select(self, opponent: Shape) -> Shape {
        match (self, opponent) {
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Draw, x) => x,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Match(pub Shape, pub Shape);

impl Match {
    fn result(self) -> MatchResult {
        match self {
            Match(Rock, Rock) => Draw,
            Match(Rock, Paper) => Win,
            Match(Rock, Scissors) => Lose,
            Match(Paper, Rock) => Lose,
            Match(Paper, Paper) => Draw,
            Match(Paper, Scissors) => Win,
            Match(Scissors, Rock) => Win,
            Match(Scissors, Paper) => Lose,
            Match(Scissors, Scissors) => Draw,
        }
    }

    fn score(self) -> usize {
        self.1.score() + self.result().score()
    }

    fn convert(self) -> Match {
        Match(
            self.0,
            (match self.1 {
                Rock => Lose,    // X
                Paper => Draw,   // Y
                Scissors => Win, // Z
            })
            .select(self.0),
        )
    }
}

pub fn solve(matches: Vec<Match>) -> usize {
    matches.iter().map(|a| a.score()).sum()
}

pub fn more(matches: Vec<Match>) -> usize {
    solve(matches.iter().map(|m| m.convert()).collect())
}
