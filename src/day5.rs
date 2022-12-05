use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct ParseMoveError;

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing move")
    }
}

impl Error for ParseMoveError {}

impl From<ParseIntError> for ParseMoveError {
    fn from(_: ParseIntError) -> Self {
        ParseMoveError
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        if words.len() < 6 {
            return Err(ParseMoveError);
        }
        Ok(Move {
            count: words[1].parse()?,
            from: words[3].parse::<usize>()? - 1,
            to: words[5].parse::<usize>()? - 1,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    // I can't make these constants because allocation is not allowed in constants
    #[allow(dead_code)]
    pub fn test_stacks() -> Self {
        /*
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
         */
        Self {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        }
    }

    pub fn stacks() -> Self {
        /*
        [M]                     [N] [Z]
        [F]             [R] [Z] [C] [C]
        [C]     [V]     [L] [N] [G] [V]
        [W]     [L]     [T] [H] [V] [F] [H]
        [T]     [T] [W] [F] [B] [P] [J] [L]
        [D] [L] [H] [J] [C] [G] [S] [R] [M]
        [L] [B] [C] [P] [S] [D] [M] [Q] [P]
        [B] [N] [J] [S] [Z] [W] [F] [W] [R]
         1   2   3   4   5   6   7   8   9
         */
        Self {
            stacks: vec![
                vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'],
                vec!['N', 'B', 'L'],
                vec!['J', 'C', 'H', 'T', 'L', 'V'],
                vec!['S', 'P', 'J', 'W'],
                vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R'],
                vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z'],
                vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'],
                vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'],
                vec!['R', 'P', 'M', 'L', 'H'],
            ],
        }
    }

    pub fn message(&self) -> Option<String> {
        self.stacks
            .iter()
            .map(|c| c.last().copied())
            .collect::<Option<String>>()
    }

    fn process_move(&mut self, mv: Move) -> Option<()> {
        for _ in 0..mv.count {
            let a = self.stacks[mv.from].pop()?;
            self.stacks[mv.to].push(a);
        }
        Some(())
    }

    pub fn process(&mut self, mv: &[Move]) -> Option<()> {
        for &mv in mv {
            self.process_move(mv)?;
        }
        Some(())
    }

    fn process_move1(&mut self, mv: Move) -> Option<()> {
        let mut s = vec![];
        for _ in 0..mv.count {
            let a = self.stacks[mv.from].pop()?;
            s.push(a);
        }
        while let Some(a) = s.pop() {
            self.stacks[mv.to].push(a);
        }
        Some(())
    }

    pub fn process1(&mut self, mv: &[Move]) -> Option<()> {
        for &mv in mv {
            self.process_move1(mv)?;
        }
        Some(())
    }
}
