#[cfg(feature = "slow")]
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

mod day1;
mod day10;
mod day11;
mod day12;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

macro_rules! slow {
    ($expr: expr) => {
        #[cfg(feature = "slow")]
        println!("{}", $expr);
        #[cfg(not(feature = "slow"))]
        println!("[skipped]");
    };
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Oopsie;

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

    {
        println!("day8");
        let mut s = String::new();
        File::open("input/day8/input.txt")?.read_to_string(&mut s)?;
        let v = s
            .split_whitespace()
            .map(|a| {
                a.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", day8::count_visible(v.clone()));
        println!("{}", day8::score(v));
    }

    {
        println!("day9");
        let mut s = String::new();
        File::open("input/day9/input.txt")?.read_to_string(&mut s)?;
        let v = s
            .trim()
            .split('\n')
            .map(|a| a.trim().split_whitespace().collect::<Vec<_>>())
            .map(|a| {
                Ok::<(char, usize), ParseIntError>((a[0].chars().next().unwrap(), a[1].parse()?))
            })
            .collect::<Result<Vec<(char, usize)>, _>>()?;
        println!("{}", day9::solve(v.clone()));
        println!("{}", day9::more(v));
    }

    {
        println!("day10");
        let mut s = String::new();
        File::open("input/day10/input.txt")?.read_to_string(&mut s)?;
        let v = s
            .trim()
            .split('\n')
            .map(|line| line.parse())
            .collect::<Result<Vec<day10::Instruction>, _>>()?;
        println!("{}", day10::solve(v.clone()));
        println!("{}", day10::more(v));
    }

    {
        println!("day11");
        let mut monkeys = day11::Monkeys::input();
        for _ in 0..20 {
            monkeys.round(3);
        }
        println!("{}", monkeys.monkey_business());
        let mut monkeys = day11::Monkeys::input();
        for _ in 0..10_000 {
            monkeys.round(1);
        }
        println!("{}", monkeys.monkey_business());
    }

    {
        println!("day12");
        let mut s = String::new();
        File::open("input/day12/input.txt")?.read_to_string(&mut s)?;
        let v: Vec<_> = s.trim().split('\n').collect();
        let mut s = (0, 0);
        let mut e = (0, 0);
        for (i, line) in v.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => s = (i, j),
                    'E' => e = (i, j),
                    _ => {}
                }
            }
        }
        let v = v
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 25,
                        _ => c as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        println!("{}", day12::solve(&v, s, e));
        println!("{}", day12::more(&v, e));
    }

    {
        println!("day14");
        let mut s = String::new();
        File::open("input/day14/input.txt")?.read_to_string(&mut s)?;
        let v = s
            .trim()
            .split('\n')
            .map(|line| {
                line.split(" -> ")
                    .map(|pair| {
                        pair.split_once(',')
                            .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
                    })
                    .collect::<Option<Vec<(usize, usize)>>>()
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(Oopsie)?;
        println!("{}", day14::solve(v.clone()));
        println!("{}", day14::more(v));
    }

    {
        println!("day15");
        let mut s = String::new();
        File::open("input/day15/input.txt")?.read_to_string(&mut s)?;
        let v: Vec<(i64, i64, i64, i64)> = s
            .split_whitespace()
            .map(|a| a.parse())
            .collect::<Result<Vec<_>, _>>()?
            .chunks_exact(4)
            .map(|a| (a[0], a[1], a[2], a[3]))
            .collect();
        println!("{}", day15::solve(v.clone(), 2000000));
        slow!(day15::more(v, 4000000));
    }

    // lol skip
    #[cfg(feature = "slow")]
    {
        println!("day16");
        let mut s = String::new();
        File::open("input/day16/input.txt")?.read_to_string(&mut s)?;
        let mut flow = HashMap::new();
        let mut tunnels = HashMap::new();
        for line in s.trim().split("\n") {
            let line = line.strip_prefix("Valve ").ok_or(Oopsie)?;
            let (valve, line) = line.split_once(" has flow rate=").ok_or(Oopsie)?;
            let (f, line) = line
                .split_once("; tunnels lead to valves ")
                .or_else(|| line.split_once("; tunnel leads to valve "))
                .ok_or(Oopsie)?;
            flow.insert(valve.to_string(), f.parse()?);
            tunnels.insert(
                valve.to_string(),
                line.split(", ").map(|a| a.to_string()).collect(),
            );
        }
        // both of these are slow :cry:
        println!("{}", day16::solve(flow.clone(), tunnels.clone()));
        println!("{}", day16::more(flow, tunnels));
    }
    #[cfg(not(feature = "slow"))]
    {
        println!("day16");
        println!("[skipped]");
        println!("[skipped]");
    }

    {
        println!("day17");
        let mut s = String::new();
        File::open("input/day17/input.txt")?.read_to_string(&mut s)?;
        println!("{}", day17::solve(s.trim()));
        println!("[unsolved]");
    }

    {
        println!("day18");
        let mut s = String::new();
        File::open("input/day18/input.txt")?.read_to_string(&mut s)?;
        let cubes: Vec<_> = s
            .split('\n')
            .map(|line| {
                let cube: Vec<_> = line.split(',').map(|a| a.parse().unwrap()).collect();
                (cube[0], cube[1], cube[2])
            })
            .collect();
        println!("{}", day18::solve(&cubes));
        println!("{}", day18::more(&cubes));
    }

    {
        println!("day19");
        println!("[unsolved]");
        println!("[unsolved]");
    }

    {
        println!("day20");
        let mut s = String::new();
        File::open("input/day20/input.txt")?.read_to_string(&mut s)?;
        let f = s
            .split('\n')
            .map(|a| a.parse())
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", day20::solve(&f));
        println!("{}", day20::more(&f));
    }

    {
        println!("day21");
        let mut s = String::new();
        File::open("input/day21/input.txt")?.read_to_string(&mut s)?;
        let monkeys = s
            .split('\n')
            .map(|line| {
                let (name, yell) = line.split_once(": ").unwrap();
                (name.to_string(), yell.parse().unwrap())
            })
            .collect();
        println!("{}", day21::solve(&monkeys));
        println!("{}", day21::more(&monkeys));
    }

    Ok(())
}
