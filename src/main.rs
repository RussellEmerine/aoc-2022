use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Oopsie;

impl Display for Oopsie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "oopsie")
    }
}

impl Error for Oopsie {}

fn main() -> Result<(), Box<dyn Error>> {
    {
        println!("day1");
        let mut s = String::new();
        File::open("input/day1/input.txt")?.read_to_string(&mut s)?;
        let calories = s
            .split("\n\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|a| a.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", day1::solve(calories.clone()));
        println!("{}", day1::more(calories));
    }

    {
        println!("day2");
        let mut s = String::new();
        File::open("input/day2/input.txt")?.read_to_string(&mut s)?;
        let matches = s
            .split("\n")
            .map(|s| {
                let match_ = s
                    .split_whitespace()
                    .map(|a| a.parse())
                    .collect::<Result<Vec<day2::Shape>, _>>()?;
                Ok::<day2::Match, day2::ParseShapeError>(day2::Match(match_[0], match_[1]))
            })
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", day2::solve(matches.clone()));
        println!("{}", day2::more(matches));
    }

    {
        println!("day3");
        let mut s = String::new();
        File::open("input/day3/input.txt")?.read_to_string(&mut s)?;
        let rucksacks: Vec<_> = s.split_whitespace().collect();
        println!("{}", day3::solve(rucksacks.clone()));
        println!("{}", day3::more(rucksacks));
    }

    {
        println!("day4");
        let mut s = String::new();
        File::open("input/day4/input.txt")?.read_to_string(&mut s)?;
        let v: Vec<_> = s
            .split_whitespace()
            .map(|a| {
                a.split(&['-', ','])
                    .map(|a| a.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|a| ((a[0], a[1]), (a[2], a[3])))
            .collect();
        println!("{}", day4::solve(v.clone()));
        println!("{}", day4::more(v));
    }

    {
        println!("day5");
        let mut s = String::new();
        File::open("input/day5/input.txt")?.read_to_string(&mut s)?;
        let mv = s
            .split('\n')
            .map(|a| a.parse())
            .collect::<Result<Vec<day5::Move>, _>>()?;
        let mut stacks = day5::Stacks::stacks();
        stacks.process(&mv).ok_or(Oopsie)?;
        println!("{}", stacks.message().ok_or(Oopsie)?);
        let mut stacks = day5::Stacks::stacks();
        stacks.process1(&mv).ok_or(Oopsie)?;
        println!("{}", stacks.message().ok_or(Oopsie)?);
    }

    {
        println!("day6");
        let mut s = String::new();
        File::open("input/day6/input.txt")?.read_to_string(&mut s)?;
        println!("{}", day6::marker::<4>(&s).ok_or(Oopsie)?);
        println!("{}", day6::marker::<14>(&s).ok_or(Oopsie)?);
    }

    {
        println!("day7");
        let mut s = String::new();
        File::open("input/day7/input.txt")?.read_to_string(&mut s)?;
        println!("{}", day7::solve(&s).ok_or(Oopsie)?);
        println!("{}", day7::more(&s).ok_or(Oopsie)?);
    }

    Ok(())
}
