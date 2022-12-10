use crate::day10::Instruction::{Addx, Noop};
use crate::Oopsie;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.trim().split_whitespace().collect();
        let (&op, args) = v.split_first().ok_or(Oopsie)?;
        match op {
            "noop" => Ok(Noop),
            "addx" => args
                .first()
                .and_then(|a| a.parse().ok())
                .map(|a| Addx(a))
                .ok_or(Oopsie),
            _ => Err(Oopsie),
        }
    }
}

pub fn solve(v: Vec<Instruction>) -> isize {
    let mut c = 1;
    let mut sum = 0;
    let mut x = 1;
    for v in v {
        if c % 40 == 20 {
            sum += x * c;
        }
        match v {
            Noop => {}
            Addx(y) => {
                c += 1;
                if c % 40 == 20 {
                    sum += x * c;
                }
                x += y;
            }
        }
        c += 1;
    }
    sum
}

#[derive(Debug, Clone)]
pub struct CRT {
    pixels: [[bool; 40]; 6],
    t: isize,
    x: usize,
    y: usize,
}

impl Display for CRT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            )?
        }
        Ok(())
    }
}

impl CRT {
    fn new() -> Self {
        Self {
            pixels: [[false; 40]; 6],
            t: 0,
            x: 0,
            y: 0,
        }
    }

    fn put(&mut self, x: isize) {
        if (x - self.y as isize).abs() <= 1 {
            self.pixels[self.x][self.y] = true;
        }
        self.t += 1;
        self.y += 1;
        if self.y == self.pixels[0].len() {
            self.y = 0;
            self.x += 1;
            if self.x == self.pixels.len() {
                self.x = 0;
            }
        }
    }
}

pub fn more(v: Vec<Instruction>) -> CRT {
    let mut crt = CRT::new();
    let mut x = 1;
    for v in v {
        crt.put(x);
        match v {
            Noop => {}
            Addx(y) => {
                crt.put(x);
                x += y;
            }
        }
    }
    crt
}
